use crate::state::AppState;
use std::process::Command;
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

#[tauri::command]
pub fn set_model(state: State<AppState>, value: String) {
    let mut model = state.model.lock().unwrap();
    *model = value;
}

#[tauri::command]
pub fn get_model(state: State<AppState>) -> String {
    let model = state.model.lock().unwrap();
    model.clone()
}
#[tauri::command]
pub fn get_models(_app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let output = Command::new("fabric")
        .arg("--listmodels")
        .output()
        .map_err(|e| format!("Failed to execute fabric command: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let models: Vec<String> = output_str
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(models)
}

#[tauri::command]
pub fn set_default_model(state: tauri::State<AppState>, modelId: i32) -> Result<(), String> {
    println!("Attempting to set default model with ID: {}", modelId);

    match state.default_pattern.lock() {
        Ok(mut default_model) => {
            println!("Successfully acquired lock on default_pattern");
            *default_model = modelId.to_string();
            println!("Successfully set default model to: {}", modelId);
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to acquire lock on default_pattern: {}", e);
            println!("Error: {}", error_msg);
            Err(error_msg)
        }
    }
}

// #[tauri::command]
// pub fn get_default_model(state: State<AppState>) -> String {
//     let default_model = state.default_pattern.lock().unwrap();
//     default_model.clone()
// }
