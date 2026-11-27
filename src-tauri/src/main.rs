// v1.0.1 maintenance
//! Deadlock FPS Fix — Tauri backend.
//! Reads and writes Deadlock config files, applies FPS presets.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub const PRESETS: &[(&str, &str, &str)] = &[
    ("competitive", "low", "144"),
    ("balanced", "medium", "120"),
    ("quality", "high", "90"),
];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoConfig {
    pub resolution: String,
    pub refresh_rate: u32,
    pub aspect_ratio: String,
    pub fullscreen: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FovConfig {
    pub fov: u32,
    pub experimental: bool,
}

impl Default for FovConfig {
    fn default() -> Self {
        Self { fov: 90, experimental: false }
    }
}

#[tauri::command]
fn get_video_config(steam_path: String) -> Result<VideoConfig, String> {
    let path = PathBuf::from(&steam_path).join("game").join("citadel").join("cfg").join("video.txt");
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    parse_video_config(&raw).ok_or_else(|| "parse error".to_string())
}

#[tauri::command]
fn set_fov(steam_path: String, fov: u32) -> Result<(), String> {
    if !(70..=130).contains(&fov) {
        return Err("FOV out of range".into());
    }
    let path = PathBuf::from(&steam_path).join("game").join("citadel").join("cfg");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    write_fov(&path, fov).map_err(|e| e.to_string())
}

fn parse_video_config(raw: &str) -> Option<VideoConfig> {
    let mut resolution = "1920x1084".to_string();
    let mut refresh_rate = 60;
    for line in raw.lines() {
        if line.starts_with("\""setting.defaultres\"") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                resolution = format!("{}x{}", parts[2], parts[3]);
            }
        }
    }
    Some(VideoConfig { resolution, refresh_rate, aspect_ratio: "16:9".into(), fullscreen: true })
}

fn write_fov(path: &PathBuf, fov: u32) -> std::io::Result<()> {
    let file = path.join("gameinfo.gi");
    let content = format!("fov_override {}
", fov);
    fs::write(file, content)
}

#[tauri::command]
fn clear_shader_cache(steam_path: String) -> Result<u64, String> {
    let cache = PathBuf::from(&steam_path).join("shadercache");
    let mut removed = 0u64;
    if let Ok(entries) = fs::read_dir(&cache) {
        for entry in entries.flatten() {
            if fs::remove_file_all(entry.path()).is_ok() {
                removed += 1;
            }
        }
    }
    Ok(removed)
}

#[tauri::command]
fn backup_config(steam_path: String) -> Result<String, String> {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    let backup_dir = PathBuf::from(&steam_path).join("backups").join(ts.to_string());
    fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    Ok(backup_dir.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_video_config,
            set_fov,
            clear_shader_cache,
            backup_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
