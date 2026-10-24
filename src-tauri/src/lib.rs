mod library;
mod player;

use std::sync::Arc;

use library::{Library, Paths};
use parking_lot::Mutex;
use player::{Command, PlayerHandle};
use serde::Serialize;
use tauri::{Manager, State};

struct AppState {
    library: Arc<Mutex<Library>>,
    paths: Arc<Paths>,
    player: PlayerHandle,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LibraryPayload {
    folders: Vec<String>,
    tracks: Vec<library::Track>,
    covers_dir: String,
}

fn payload(state: &AppState) -> LibraryPayload {
    let lib = state.library.lock();
    LibraryPayload {
        folders: lib.folders.clone(),
        tracks: lib.tracks.clone(),
        covers_dir: state.paths.covers_dir.to_string_lossy().to_string(),
    }
}

/// Scans off the main thread so the window never freezes.
async fn rescan_inner(state: &AppState) -> Result<LibraryPayload, String> {
    let library = state.library.clone();
    let paths = state.paths.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let mut snapshot = library.lock().clone();
        snapshot.rescan(&paths);
        let _ = snapshot.save(&paths);
        *library.lock() = snapshot;
    })
    .await
    .map_err(|e| e.to_string())?;
    Ok(payload(state))
}

#[tauri::command]
fn get_library(state: State<'_, AppState>) -> LibraryPayload {
    payload(&state)
}

#[tauri::command]
async fn add_folder(path: String, state: State<'_, AppState>) -> Result<LibraryPayload, String> {
    {
        let mut lib = state.library.lock();
        if !lib.folders.contains(&path) {
            lib.folders.push(path);
        }
    }
    rescan_inner(&state).await
}

#[tauri::command]
async fn remove_folder(path: String, state: State<'_, AppState>) -> Result<LibraryPayload, String> {
    state.library.lock().folders.retain(|f| f != &path);
    rescan_inner(&state).await
}

#[tauri::command]
async fn rescan(state: State<'_, AppState>) -> Result<LibraryPayload, String> {
    rescan_inner(&state).await
}

#[tauri::command]
fn player_load(path: String, autoplay: bool, start: Option<f64>, state: State<'_, AppState>) {
    state.player.send(Command::Load {
        path,
        autoplay,
        start: start.unwrap_or(0.0),
    });
}

#[tauri::command]
fn player_play(state: State<'_, AppState>) {
    state.player.send(Command::Play);
}

#[tauri::command]
fn player_pause(state: State<'_, AppState>) {
    state.player.send(Command::Pause);
}

#[tauri::command]
fn player_seek(position: f64, state: State<'_, AppState>) {
    state.player.send(Command::Seek(position));
}

#[tauri::command]
fn player_volume(volume: f32, state: State<'_, AppState>) {
    state.player.send(Command::Volume(volume));
}

#[tauri::command]
fn player_stop(state: State<'_, AppState>) {
    state.player.send(Command::Stop);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;
            let cache_dir = app.path().app_cache_dir()?;
            let paths = Paths {
                library_file: data_dir.join("library.json"),
                covers_dir: cache_dir.join("covers"),
            };
            std::fs::create_dir_all(&paths.covers_dir)?;
            // Covers are served to the WebView straight from disk via the asset protocol.
            app.asset_protocol_scope()
                .allow_directory(&paths.covers_dir, false)?;

            let library = Library::load(&paths);
            app.manage(AppState {
                library: Arc::new(Mutex::new(library)),
                paths: Arc::new(paths),
                player: PlayerHandle::spawn(app.handle().clone()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_library,
            add_folder,
            remove_folder,
            rescan,
            player_load,
            player_play,
            player_pause,
            player_seek,
            player_volume,
            player_stop
        ])
        .run(tauri::generate_context!())
        .expect("error while running Lyra");
}
