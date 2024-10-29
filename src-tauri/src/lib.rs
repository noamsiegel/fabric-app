use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

pub mod fabric;
use crate::fabric::patterns::{
    get_fabric_dir, get_patterns, get_selected_pattern, set_selected_pattern,
};
use crate::fabric::run::{
    clipboard_contents_and_run_pattern, get_is_running, scrape_question_and_run_pattern,
    scrape_url_and_run_pattern, set_is_running,
};
use crate::fabric::secrets::{
    get_api_keys, get_base_urls, get_env_file_path, get_secret, update_secret,
};
use crate::fabric::settings::{
    get_model, get_models, get_presence_penalty, get_temperature, set_default_model, set_model,
    set_presence_penalty, set_temperature,
};

pub mod plugins;
use crate::plugins::get_clipboard_contents;

mod state;
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            // Initialize the AppState with an empty fabric folder
            app.manage(AppState {
                fabric_folder: Mutex::new(String::new()),
                fabric_dir: Mutex::new(PathBuf::new()),
                default_pattern: Mutex::new(String::new()),
                selected_pattern: Mutex::new(String::new()),
                patterns: Mutex::new(Vec::new()),
                is_running: Mutex::new(false),
                // fabric pattern flags
                temperature: Mutex::new(0.7),
                presence_penalty: Mutex::new(0.0),
                default_model: Mutex::new(String::new()),
                frequency_penalty: Mutex::new(0.0),
                model: Mutex::new(String::new()),
                top_p: Mutex::new(0.9),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // runs
            scrape_url_and_run_pattern,
            scrape_question_and_run_pattern,
            clipboard_contents_and_run_pattern,
            get_is_running,
            set_is_running,
            // patterns
            get_fabric_dir,
            get_patterns,
            set_selected_pattern,
            get_selected_pattern,
            // secrets
            get_env_file_path,
            get_api_keys,
            get_base_urls,
            get_secret,
            update_secret,
            // fabric LLM flags
            get_clipboard_contents,
            set_temperature,
            get_temperature,
            set_presence_penalty,
            get_presence_penalty,
            get_models,
            get_model,
            set_model,
            set_default_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
