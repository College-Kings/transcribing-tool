use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Mutex};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub episode: Mutex<i32>,
    pub selected_file: Mutex<Option<PathBuf>>,
    pub selected_folder: Mutex<Option<PathBuf>>,
    pub scene_number: Mutex<Option<String>>,
}

impl Settings {
    pub fn new(episode: i32) -> Self {
        Self {
            episode: Mutex::new(episode),
            selected_file: Mutex::new(None),
            selected_folder: Mutex::new(None),
            scene_number: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub fn get_episode_number(settings: State<Settings>) -> i32 {
    *settings.episode.lock().unwrap()
}

#[tauri::command]
pub fn set_episode_number(settings: State<Settings>, value: i32) {
    *settings.episode.lock().unwrap() = value;
    println!("Episode number set to: {}", value)
}

pub fn initialise_settings() -> Result<Settings, Error> {
    let settings_file = dirs::config_local_dir()
        .unwrap()
        .join("Viridian")
        .join("TranscribingTool")
        .join("settings.json");

    if settings_file.exists() {
        let settings = fs::read_to_string(&settings_file)?;
        let settings: Settings = serde_json::from_str(&settings).unwrap_or(Settings::new(1));

        return Ok(settings);
    }

    let settings = Settings::new(1);

    fs::create_dir_all(settings_file.parent().unwrap())?;
    fs::write(&settings_file, serde_json::to_string(&settings)?)?;

    Ok(settings)
}
