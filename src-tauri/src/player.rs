//! Native audio playback on a dedicated thread (rodio + symphonia).
//! The UI only sends commands and receives small progress events,
//! so no audio data ever goes through the WebView.

use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player};
use serde::Serialize;
use tauri::{AppHandle, Emitter};

pub enum Command {
    Load {
        path: String,
        autoplay: bool,
        start: f64,
    },
    Play,
    Pause,
    Seek(f64),
    Volume(f32),
    Stop,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Progress {
    position: f64,
    playing: bool,
}

#[derive(Clone)]
pub struct PlayerHandle {
    tx: Sender<Command>,
}

impl PlayerHandle {
    pub fn spawn(app: AppHandle) -> Self {
        let (tx, rx) = mpsc::channel();
        thread::Builder::new()
            .name("lyra-audio".into())
            .spawn(move || audio_thread(app, rx))
            .expect("spawn audio thread");
        Self { tx }
    }

    pub fn send(&self, cmd: Command) {
        let _ = self.tx.send(cmd);
    }
}

struct Engine {
    // The device sink must outlive the player; it's opened lazily on first use.
    sink: Option<MixerDeviceSink>,
    player: Option<Player>,
    volume: f32,
    /// True while a track is loaded and hasn't reached its end.
    active: bool,
}

impl Engine {
    fn ensure_player(&mut self) -> Result<&Player, String> {
        if self.sink.is_none() {
            let mut sink = DeviceSinkBuilder::open_default_sink().map_err(|e| e.to_string())?;
            sink.log_on_drop(false);
            self.sink = Some(sink);
        }
        if self.player.is_none() {
            let player = Player::connect_new(self.sink.as_ref().unwrap().mixer());
            player.set_volume(self.volume * self.volume);
            self.player = Some(player);
        }
        Ok(self.player.as_ref().unwrap())
    }

    fn load(&mut self, path: &str, autoplay: bool, start: f64) -> Result<(), String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let len = file.metadata().map(|m| m.len()).ok();
        let mut builder = Decoder::builder()
            .with_data(BufReader::new(file))
            .with_seekable(true)
            .with_gapless(true);
        if let Some(len) = len {
            builder = builder.with_byte_len(len);
        }
        if let Some(ext) = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
        {
            builder = builder.with_hint(ext);
        }
        let source = builder.build().map_err(|e| e.to_string())?;

        let player = self.ensure_player()?;
        player.clear();
        player.append(source);
        if start > 0.0 {
            let _ = player.try_seek(Duration::from_secs_f64(start));
        }
        if autoplay {
            player.play();
        } else {
            player.pause();
        }
        self.active = true;
        Ok(())
    }
}

fn audio_thread(app: AppHandle, rx: Receiver<Command>) {
    let mut engine = Engine {
        sink: None,
        player: None,
        volume: 0.8,
        active: false,
    };
    let mut last_sent: Option<(u64, bool)> = None;

    loop {
        match rx.recv_timeout(Duration::from_millis(250)) {
            Ok(cmd) => match cmd {
                Command::Load {
                    path,
                    autoplay,
                    start,
                } => {
                    if let Err(err) = engine.load(&path, autoplay, start) {
                        engine.active = false;
                        let _ = app.emit("player:error", format!("{path}: {err}"));
                    }
                }
                Command::Play => {
                    if let Some(p) = &engine.player {
                        p.play();
                    }
                }
                Command::Pause => {
                    if let Some(p) = &engine.player {
                        p.pause();
                    }
                }
                Command::Seek(secs) => {
                    if let Some(p) = &engine.player {
                        let _ = p.try_seek(Duration::from_secs_f64(secs.max(0.0)));
                    }
                }
                Command::Volume(v) => {
                    engine.volume = v.clamp(0.0, 1.0);
                    if let Some(p) = &engine.player {
                        // Perceptual curve: linear sliders feel wrong for loudness.
                        p.set_volume(engine.volume * engine.volume);
                    }
                }
                Command::Stop => {
                    if let Some(p) = &engine.player {
                        p.clear();
                    }
                    engine.active = false;
                }
            },
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }

        let Some(player) = &engine.player else {
            continue;
        };

        if engine.active && player.empty() {
            engine.active = false;
            let _ = app.emit("player:ended", ());
            continue;
        }

        let playing = engine.active && !player.is_paused();
        let pos = player.get_pos().as_secs_f64();
        // Only emit when something changed (~4 events/s while playing, none while paused).
        let key = ((pos * 4.0) as u64, playing);
        if last_sent != Some(key) {
            last_sent = Some(key);
            let _ = app.emit(
                "player:progress",
                Progress {
                    position: pos,
                    playing,
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Plays a real file through the default output device when `LYRA_TEST_AUDIO` points to one.
    /// Checks are done while paused, so they don't depend on how fast the device consumes audio.
    #[test]
    fn plays_and_seeks() {
        let Ok(path) = std::env::var("LYRA_TEST_AUDIO") else {
            return;
        };
        let mut engine = Engine {
            sink: None,
            player: None,
            volume: 0.5,
            active: false,
        };

        // Loading paused at an offset (session restore) keeps the position.
        engine.load(&path, false, 1.5).expect("load");
        thread::sleep(Duration::from_millis(200));
        let player = engine.player.as_ref().unwrap();
        assert!(player.is_paused());
        let pos = player.get_pos().as_secs_f64();
        assert!((1.4..1.7).contains(&pos), "restored at {pos}");

        player.try_seek(Duration::from_secs(3)).expect("seek");
        thread::sleep(Duration::from_millis(200));
        let pos = player.get_pos().as_secs_f64();
        assert!((2.9..3.2).contains(&pos), "seek landed at {pos}");

        player.play();
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while !player.empty() && std::time::Instant::now() < deadline {
            thread::sleep(Duration::from_millis(50));
        }
        assert!(player.empty(), "track should finish");
        assert!(player.get_pos().as_secs_f64() >= 3.9);
    }
}
