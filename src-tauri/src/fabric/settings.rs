use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub fn set_temperature(state: State<AppState>, value: f32) {
    let mut temperature = state.temperature.lock().unwrap();
    *temperature = value;
}

#[tauri::command]
pub fn get_temperature(state: State<AppState>) -> f32 {
    let temperature = state.temperature.lock().unwrap();
    *temperature
}

#[tauri::command]
pub fn set_presence_penalty(state: State<AppState>, value: f32) {
    let mut presence_penalty = state.presence_penalty.lock().unwrap();
    *presence_penalty = value;
}

#[tauri::command]
pub fn get_presence_penalty(state: State<AppState>) -> f32 {
    let presence_penalty = state.presence_penalty.lock().unwrap();
    *presence_penalty
}
