use std::sync::Mutex;
use tauri::Manager;

pub mod fabric;
use crate::fabric::patterns::{
    get_fabric_folder, get_patterns, get_selected_pattern, set_fabric_folder, set_patterns,
    set_selected_pattern,
};
use crate::fabric::settings::{
    get_presence_penalty, get_temperature, set_presence_penalty, set_temperature,
};

pub mod plugins;
use crate::plugins::get_clipboard_contents;

mod state;
use crate::state::{get_is_running, set_is_running, AppState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // Initialize the AppState with an empty fabric folder
            app.manage(AppState {
                fabric_folder: Mutex::new(String::new()),
                default_pattern: Mutex::new(String::new()),
                selected_pattern: Mutex::new(String::new()),
                patterns: Mutex::new(Vec::new()),
                is_running: Mutex::new(false),
                temperature: Mutex::new(0.7),
                presence_penalty: Mutex::new(0.0),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            set_fabric_folder,
            get_fabric_folder,
            get_patterns,
            set_selected_pattern,
            get_selected_pattern,
            set_patterns,
            get_is_running,
            set_is_running,
            get_clipboard_contents,
            set_temperature,
            get_temperature,
            set_presence_penalty,
            get_presence_penalty,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
