use std::fs;
use std::path::Path;
use std::sync::Mutex;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Define the AppState struct
struct AppState {
    fabric_folder: Mutex<String>,
}

// New command to set the fabric folder
#[tauri::command]
fn set_fabric_folder(state: tauri::State<AppState>, path: String) {
    let mut fabric_folder = state.fabric_folder.lock().unwrap();
    *fabric_folder = path;
}

#[tauri::command]
fn get_fabric_folder(state: tauri::State<AppState>) -> String {
    let fabric_folder = state.fabric_folder.lock().unwrap();
    fabric_folder.clone()
}

#[tauri::command]
fn get_patterns(state: tauri::State<AppState>) -> Result<Vec<String>, String> {
    let fabric_folder = state.fabric_folder.lock().unwrap();

    if fabric_folder.is_empty() {
        return Err("Fabric folder not set".to_string());
    }

    let path = Path::new(&*fabric_folder).join("patterns");

    if !path.exists() {
        return Err(format!(
            "Patterns directory does not exist: {}",
            path.display()
        ));
    }

    Ok(fs::read_dir(path)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    e.file_name().into_string().ok()
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the AppState with an empty fabric folder
            app.manage(AppState {
                fabric_folder: Mutex::new(String::new()),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_fabric_folder,
            get_fabric_folder,
            get_patterns,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
