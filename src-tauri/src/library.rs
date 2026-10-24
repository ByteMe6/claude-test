//! Music library: scanning folders, reading tags, caching cover art.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use lofty::picture::PictureType;
use lofty::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "ogg", "oga", "m4a", "mp4", "aac", "wav", "aiff", "aif", "alac", "caf", "mka",
    "webm",
];

/// Covers are downscaled to this size, so the UI never decodes 3000px scans.
const COVER_SIZE: u32 = 512;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: String,
    pub path: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub album_id: String,
    pub track_no: Option<u32>,
    pub disc_no: Option<u32>,
    pub year: Option<u16>,
    pub genre: Option<String>,
    pub duration: f64,
    /// File name of the cached cover inside the covers directory.
    pub cover: Option<String>,
    /// Dominant cover color as "r,g,b" — used for the UI gradients.
    pub color: Option<String>,
    pub mtime: u64,
    pub added: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub folders: Vec<String>,
    pub tracks: Vec<Track>,
}

pub struct Paths {
    pub library_file: PathBuf,
    pub covers_dir: PathBuf,
}

impl Library {
    pub fn load(paths: &Paths) -> Self {
        fs::read(&paths.library_file)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, paths: &Paths) -> std::io::Result<()> {
        if let Some(dir) = paths.library_file.parent() {
            fs::create_dir_all(dir)?;
        }
        let tmp = paths.library_file.with_extension("json.tmp");
        fs::write(&tmp, serde_json::to_vec(self)?)?;
        fs::rename(tmp, &paths.library_file)
    }

    /// Rescans every folder. Files whose mtime didn't change are reused as-is,
    /// so a rescan of a big library only touches new or edited files.
    pub fn rescan(&mut self, paths: &Paths) {
        let _ = fs::create_dir_all(&paths.covers_dir);
        let known: HashMap<&str, &Track> =
            self.tracks.iter().map(|t| (t.path.as_str(), t)).collect();

        let files: Vec<PathBuf> = {
            let mut seen = HashSet::new();
            self.folders
                .iter()
                .flat_map(|folder| {
                    WalkDir::new(folder)
                        .follow_links(true)
                        .into_iter()
                        .filter_map(Result::ok)
                        .filter(|e| e.file_type().is_file() && is_audio(e.path()))
                        .map(|e| e.into_path())
                })
                .filter(|p| seen.insert(p.clone()))
                .collect()
        };

        let now = unix_now();
        let tracks: Vec<Track> = files
            .par_iter()
            .filter_map(|path| {
                let path_str = path.to_string_lossy().to_string();
                let mtime = file_mtime(path);
                if let Some(old) = known.get(path_str.as_str()) {
                    if old.mtime == mtime {
                        return Some((*old).clone());
                    }
                }
                let added = known.get(path_str.as_str()).map_or(now, |t| t.added);
                read_track(path, mtime, added, &paths.covers_dir)
            })
            .collect();

        let mut tracks = tracks;
        tracks.sort_by(|a, b| {
            (
                a.album_artist.to_lowercase(),
                a.album.to_lowercase(),
                a.disc_no,
                a.track_no,
            )
                .cmp(&(
                    b.album_artist.to_lowercase(),
                    b.album.to_lowercase(),
                    b.disc_no,
                    b.track_no,
                ))
        });
        self.tracks = tracks;
    }
}

fn is_audio(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| AUDIO_EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
        .unwrap_or(false)
}

fn file_mtime(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_secs())
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

fn short_hash(input: &str) -> String {
    blake3::hash(input.as_bytes()).to_hex()[..16].to_string()
}

fn read_track(path: &Path, mtime: u64, added: u64, covers_dir: &Path) -> Option<Track> {
    let tagged = lofty::read_from_path(path).ok()?;
    let duration = tagged.properties().duration().as_secs_f64();
    let tag = tagged.primary_tag().or_else(|| tagged.first_tag());

    let file_stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let folder_name = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown Album".into());

    let text = |v: Option<std::borrow::Cow<'_, str>>| {
        v.map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
    };

    let title = tag.and_then(|t| text(t.title())).unwrap_or(file_stem);
    let artist = tag
        .and_then(|t| text(t.artist()))
        .unwrap_or_else(|| "Unknown Artist".into());
    let album = tag.and_then(|t| text(t.album())).unwrap_or(folder_name);
    let album_artist = tag
        .and_then(|t| t.get_string(ItemKey::AlbumArtist))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| artist.clone());

    let album_id = short_hash(&format!(
        "{}\u{1f}{}",
        album_artist.to_lowercase(),
        album.to_lowercase()
    ));

    let (cover, color) = tag
        .and_then(|t| {
            t.pictures()
                .iter()
                .find(|p| p.pic_type() == PictureType::CoverFront)
                .or_else(|| t.pictures().first())
                .map(|p| p.data().to_vec())
        })
        .or_else(|| folder_cover(path))
        .and_then(|bytes| cache_cover(&album_id, &bytes, covers_dir))
        .map_or((None, None), |(c, col)| (Some(c), Some(col)));

    Some(Track {
        id: short_hash(&path.to_string_lossy()),
        path: path.to_string_lossy().to_string(),
        title,
        artist,
        album,
        album_artist,
        album_id,
        track_no: tag.and_then(|t| t.track()),
        disc_no: tag.and_then(|t| t.disk()),
        year: tag.and_then(|t| t.date()).map(|d| d.year),
        genre: tag.and_then(|t| text(t.genre())),
        duration,
        cover,
        color,
        mtime,
        added,
    })
}

/// Looks for cover.jpg / folder.png / etc. next to the file.
fn folder_cover(path: &Path) -> Option<Vec<u8>> {
    let dir = path.parent()?;
    for name in ["cover", "folder", "front", "album", "artwork"] {
        for ext in ["jpg", "jpeg", "png", "webp"] {
            for candidate in [
                format!("{name}.{ext}"),
                format!("{}.{ext}", capitalize(name)),
            ] {
                if let Ok(bytes) = fs::read(dir.join(candidate)) {
                    return Some(bytes);
                }
            }
        }
    }
    None
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
        .unwrap_or_default()
}

/// Writes a downscaled JPEG for the album (once) and returns its file name and dominant color.
fn cache_cover(album_id: &str, bytes: &[u8], covers_dir: &Path) -> Option<(String, String)> {
    let file_name = format!("{album_id}.jpg");
    let color_file = covers_dir.join(format!("{album_id}.color"));
    let out = covers_dir.join(&file_name);

    if out.exists() {
        if let Ok(color) = fs::read_to_string(&color_file) {
            return Some((file_name, color));
        }
    }

    let img = image::load_from_memory(bytes).ok()?;
    let thumb = img.thumbnail(COVER_SIZE, COVER_SIZE).to_rgb8();
    let color = dominant_color(&thumb);

    // Several tracks of one album may race here; writing to a unique temp file keeps it safe.
    let tmp = covers_dir.join(format!(
        "{album_id}.{}.tmp",
        rayon::current_thread_index().unwrap_or(0)
    ));
    thumb
        .save_with_format(&tmp, image::ImageFormat::Jpeg)
        .ok()?;
    fs::rename(&tmp, &out).ok()?;
    let _ = fs::write(&color_file, &color);
    Some((file_name, color))
}

/// A vivid "dominant" color: averages pixels weighted by saturation,
/// then clamps brightness so it works as a dark-UI accent.
fn dominant_color(img: &image::RgbImage) -> String {
    let small = image::imageops::thumbnail(img, 32, 32);
    let (mut r, mut g, mut b, mut w) = (0f64, 0f64, 0f64, 0f64);
    for p in small.pixels() {
        let [pr, pg, pb] = p.0.map(|c| c as f64);
        let max = pr.max(pg).max(pb);
        let min = pr.min(pg).min(pb);
        let sat = if max == 0.0 { 0.0 } else { (max - min) / max };
        let lum = max / 255.0;
        // Prefer saturated, mid-bright pixels.
        let weight = 0.05 + sat * sat * (1.0 - (lum - 0.6).abs());
        r += pr * weight;
        g += pg * weight;
        b += pb * weight;
        w += weight;
    }
    let (mut r, mut g, mut b) = (r / w, g / w, b / w);
    let max = r.max(g).max(b).max(1.0);
    let target = max.clamp(90.0, 200.0);
    let k = target / max;
    r *= k;
    g *= k;
    b *= k;
    format!(
        "{},{},{}",
        r.round() as u8,
        g.round() as u8,
        b.round() as u8
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Scans a real folder when `LYRA_TEST_MUSIC` is set (skipped otherwise).
    #[test]
    fn scans_folder() {
        let Ok(dir) = std::env::var("LYRA_TEST_MUSIC") else {
            return;
        };
        let tmp = std::env::temp_dir().join(format!("lyra-test-{}", std::process::id()));
        let paths = Paths {
            library_file: tmp.join("library.json"),
            covers_dir: tmp.join("covers"),
        };
        let mut lib = Library {
            folders: vec![dir],
            tracks: vec![],
        };
        lib.rescan(&paths);
        lib.save(&paths).unwrap();
        for t in &lib.tracks {
            println!(
                "{:>2?} | {} | {} | {} | {:.1}s | cover={:?} color={:?}",
                t.track_no, t.title, t.artist, t.album, t.duration, t.cover, t.color
            );
            // Every file must also be decodable by the playback engine.
            let file = std::fs::File::open(&t.path).unwrap();
            rodio::Decoder::try_from(file).unwrap_or_else(|e| panic!("{}: {e}", t.path));
        }
        assert!(!lib.tracks.is_empty());
        assert_eq!(Library::load(&paths).tracks.len(), lib.tracks.len());
        let _ = std::fs::remove_dir_all(tmp);
    }

    #[test]
    fn dominant_color_is_bright_enough() {
        let img = image::RgbImage::from_pixel(8, 8, image::Rgb([10, 20, 60]));
        let c = dominant_color(&img);
        let max = c
            .split(',')
            .map(|v| v.parse::<u8>().unwrap())
            .max()
            .unwrap();
        assert!(max >= 90, "{c}");
    }
}
