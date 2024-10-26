use crate::state::AppState;
use std::fs;
use std::path::Path;

// New command to set the fabric folder
#[tauri::command]
pub fn set_fabric_folder(state: tauri::State<AppState>, path: String) {
    let mut fabric_folder = state.fabric_folder.lock().unwrap();
    *fabric_folder = path;
}

#[tauri::command]
pub fn get_fabric_folder(state: tauri::State<AppState>) -> String {
    let fabric_folder = state.fabric_folder.lock().unwrap();
    fabric_folder.clone()
}

#[tauri::command]
pub fn set_patterns(state: tauri::State<AppState>) -> Result<(), String> {
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

    let patterns = fs::read_dir(path)
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
        .collect::<Vec<String>>();

    // Save patterns to the state
    let mut state_patterns = state.patterns.lock().unwrap();
    *state_patterns = patterns;

    Ok(())
}

#[tauri::command]
pub fn get_patterns(state: tauri::State<AppState>) -> Vec<String> {
    let patterns = state.patterns.lock().unwrap();
    patterns.clone()
}

#[tauri::command]
pub fn set_selected_pattern(state: tauri::State<AppState>, pattern: String) {
    let mut selected_pattern = state.selected_pattern.lock().unwrap();
    *selected_pattern = pattern;
}

#[tauri::command]
pub fn get_selected_pattern(state: tauri::State<AppState>) -> String {
    let selected_pattern = state.selected_pattern.lock().unwrap();
    selected_pattern.clone()
}
