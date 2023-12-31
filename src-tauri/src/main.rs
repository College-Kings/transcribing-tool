// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod error;
mod file_formatter;
mod render_table_creator;
mod settings;

use commands::{convert_file, create_render_table, file_dialogue, greet};
use error::Error;
use settings::{get_episode_number, initialise_settings, set_episode_number};

fn main() -> Result<(), Error> {
    let settings = initialise_settings();

    tauri::Builder::default()
        .manage(settings?)
        .invoke_handler(tauri::generate_handler![
            greet,
            file_dialogue,
            convert_file,
            get_episode_number,
            set_episode_number,
            create_render_table
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
