// pub mod fabric::secrets;
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

#[tauri::command]
pub fn set_top_p(state: State<AppState>, value: f32) {
    let mut top_p = state.top_p.lock().unwrap();
    *top_p = value;
}

#[tauri::command]
pub fn get_top_p(state: State<AppState>) -> f32 {
    let top_p = state.top_p.lock().unwrap();
    *top_p
}

#[tauri::command]
pub fn set_frequency_penalty(state: State<AppState>, value: f32) {
    let mut frequency_penalty = state.frequency_penalty.lock().unwrap();
    *frequency_penalty = value;
}

#[tauri::command]
pub fn get_frequency_penalty(state: State<AppState>) -> f32 {
    let frequency_penalty = state.frequency_penalty.lock().unwrap();
    *frequency_penalty
}
