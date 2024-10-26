use std::sync::Mutex;

pub struct AppState {
    pub fabric_folder: Mutex<String>,
    pub default_pattern: Mutex<String>,
    pub selected_pattern: Mutex<String>,
    pub patterns: Mutex<Vec<String>>,
    pub is_running: Mutex<bool>,
    pub temperature: Mutex<f32>,
    pub presence_penalty: Mutex<f32>,
}

#[tauri::command]
pub fn get_is_running(state: tauri::State<AppState>) -> bool {
    let is_running = state.is_running.lock().unwrap();
    *is_running
}

#[tauri::command]
pub fn set_is_running(state: tauri::State<AppState>, value: bool) {
    let mut is_running = state.is_running.lock().unwrap();
    *is_running = value;
}
