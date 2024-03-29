use crate::settings::Settings;
use crate::utils::get_files_from_dir;
use crate::{render_table_creator, transcribing_formatter, writing_formatter};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
pub fn file_dialogue(settings: State<Settings>, select_folder: bool) -> String {
    fn update_episode_number(path: &Path, settings: &State<Settings>) {
        for component in path.components() {
            let component = component.as_os_str().to_str().unwrap();
            if component.starts_with("ep") {
                *settings.episode.lock().unwrap() = component.replace("ep", "").parse().unwrap();
            }
        }
    }

    if select_folder {
        *settings.selected_file.lock().unwrap() = None;
        let folder_path = FileDialogBuilder::new().pick_folder();

        if let Some(path) = &folder_path {
            update_episode_number(path, &settings);
        }
        *settings.selected_folder.lock().unwrap() = folder_path;
    } else {
        *settings.selected_folder.lock().unwrap() = None;

        let file_path = FileDialogBuilder::new()
            .add_filter("Renpy Files (*.rpy)", &["rpy"])
            .add_filter("All Files", &["*"])
            .pick_file();

        if let Some(path) = &file_path {
            update_episode_number(path, &settings);
        }
        *settings.selected_file.lock().unwrap() = file_path
    }

    if select_folder {
        match *settings.selected_folder.lock().unwrap() {
            Some(ref path) => format!("Selected folder: {}", path.to_str().unwrap()),
            None => "No folder selected".into(),
        }
    } else {
        match *settings.selected_file.lock().unwrap() {
            Some(ref path) => format!("Selected file: {}", path.to_str().unwrap()),
            None => "No file selected".into(),
        }
    }
    // TODO: Toggle "Covert File" button to visible
}

#[tauri::command]
pub fn run_writing_formatter(settings: State<Settings>) {
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    let path = match (selected_file, selected_folder) {
        (Some(path), None) => path,
        (None, Some(path)) => path,
        _ => return,
    };

    let speakers = match settings.get_speakers(&path) {
        Ok(speakers) => speakers,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if path.is_dir() {
        let files = get_files_from_dir(path);

        for file in files {
            writing_formatter::process_single_file(&speakers, &file)
                .expect("Unable to convert file");
            println!("Converted file: {}", file.to_str().unwrap())
        }
    } else {
        writing_formatter::process_single_file(&speakers, &path).expect("Unable to convert file");
        println!("Converted file: {}", path.to_str().unwrap())
    }
}

#[tauri::command]
pub fn run_transcribing_formatter(settings: State<Settings>) {
    // TODO: Log conversion progress to main window;

    let episode = *settings.episode.lock().unwrap();
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    if let Some(path) = selected_file {
        transcribing_formatter::process_single_file(episode, &path)
            .expect("Unable to convert file");
        println!("Converted file: {}", path.to_str().unwrap())
    } else if let Some(path) = selected_folder {
        let files = get_files_from_dir(path);

        for file in files {
            transcribing_formatter::process_single_file(episode, &file)
                .expect("Unable to convert file");
            println!("Converted file: {}", file.to_str().unwrap())
        }
    }
}

#[tauri::command]
pub fn create_render_table(settings: State<Settings>) {
    let selected_file = settings.selected_file.lock().unwrap().clone();
    let selected_folder = settings.selected_folder.lock().unwrap().clone();

    match (selected_file, selected_folder) {
        (Some(path), None) => {
            if render_table_creator::process_single_file(path.clone()).is_ok() {
                println!("Created render table: {}", path.to_str().unwrap())
            }
        }
        (None, Some(path)) => {
            let files = fs::read_dir(path)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .collect::<Vec<PathBuf>>();

            for file in files {
                if render_table_creator::process_single_file(file.clone()).is_ok() {
                    println!("Created render table: {}", file.to_str().unwrap())
                }
            }
        }
        _ => {}
    }
}
