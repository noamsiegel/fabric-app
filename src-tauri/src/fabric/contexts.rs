use crate::fabric::patterns::run_fabric;
use tauri::{AppHandle, Error};

#[tauri::command]
pub async fn set_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--context={}", context)).await
}

#[tauri::command]
pub async fn list_contexts(app: AppHandle) -> Result<String, Error> {
    run_fabric(app, "--listcontexts".to_string()).await
}

#[tauri::command]
pub async fn wipe_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--wipecontext={}", context)).await
}

#[tauri::command]
pub async fn print_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--printcontext={}", context)).await
}
