use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub port: u16,
    pub api_key: String,
    pub auto_start: bool,
    pub bind_address: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            port: 8743,
            api_key: generate_api_key(),
            auto_start: false,
            bind_address: "0.0.0.0".to_string(),
        }
    }
}

pub struct AppState {
    pub settings: Arc<Mutex<Settings>>,
    pub server_shutdown_tx: Mutex<Option<oneshot::Sender<()>>>,
}

fn generate_api_key() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn regenerate_key() -> String {
    generate_api_key()
}

fn settings_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let app_dir = config_dir.join("cross-paste");
    fs::create_dir_all(&app_dir).ok();
    app_dir.join("settings.json")
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    if path.exists() {
        let data = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        let settings = Settings::default();
        save_settings(&settings).ok();
        settings
    }
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path();
    let data = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&path, data).map_err(|e| e.to_string())
}
