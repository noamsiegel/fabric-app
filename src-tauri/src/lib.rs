use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

pub mod fabric;
use crate::fabric::install::install_fabric;
use crate::fabric::patterns::{
    get_default_pattern, get_fabric_dir, get_patterns, get_patterns_git_folder,
    get_patterns_git_repo, get_selected_pattern, set_default_pattern, set_patterns_git_folder,
    set_patterns_git_repo, set_selected_pattern, update_patterns,
};
use crate::fabric::run::{
    clipboard_contents_and_run_pattern, get_is_running, scrape_question_and_run_pattern,
    scrape_url_and_run_pattern, set_is_running,
};
use crate::fabric::secrets::{
    get_api_keys, get_base_urls, get_env_file_path, get_secret, get_secrets, reset_secret,
    update_secret,
};
use crate::fabric::settings::model_parameters::{
    get_frequency_penalty, get_presence_penalty, get_temperature, get_top_p, set_frequency_penalty,
    set_presence_penalty, set_temperature, set_top_p,
};
use crate::fabric::settings::models::{get_models, get_vendors, refresh_models};

pub mod plugins;
use crate::plugins::get_clipboard_contents;

mod state;
use crate::state::AppState;

use crate::fabric::sessions::{
    list_sessions, output_session, print_session, set_session, wipe_session,
};

use crate::fabric::contexts::{
    create_context_file, delete_context_file, get_contexts_dir, list_contexts, print_context,
    read_context_file, save_context_file, set_context, set_current_context, wipe_context,
};

use crate::fabric::paths::*;

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
            install_fabric,
            get_fabric_dir,
            // runs
            scrape_url_and_run_pattern,
            scrape_question_and_run_pattern,
            clipboard_contents_and_run_pattern,
            get_is_running,
            set_is_running,
            // patterns
            get_patterns,
            update_patterns,
            set_selected_pattern,
            get_selected_pattern,
            get_patterns_git_repo,
            get_patterns_git_folder,
            set_patterns_git_repo,
            set_patterns_git_folder,
            get_default_pattern,
            set_default_pattern,
            // vendors
            get_vendors,
            // secrets
            get_env_file_path,
            get_api_keys,
            get_base_urls,
            get_secret,
            update_secret,
            get_secrets,
            reset_secret,
            // fabric LLM flags
            get_clipboard_contents,
            set_temperature,
            get_temperature,
            set_presence_penalty,
            get_presence_penalty,
            set_top_p,
            get_top_p,
            set_frequency_penalty,
            get_frequency_penalty,
            // models
            get_models,
            refresh_models,
            // sessions
            list_sessions,
            output_session,
            set_session,
            wipe_session,
            print_session,
            // contexts
            list_contexts,
            print_context,
            set_context,
            wipe_context,
            get_contexts_dir,
            create_context_file,
            read_context_file,
            save_context_file,
            delete_context_file,
            set_current_context,
            // paths
            get_home_dir,
            get_fabric_config_dir,
            get_fabric_bin_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
