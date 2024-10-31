use crate::fabric::patterns::run_fabric;
use std::path::PathBuf;
use tauri::{AppHandle, Error, Manager};

#[tauri::command]
pub async fn set_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--context={}", context))
}

#[tauri::command]
pub async fn list_contexts(app: AppHandle) -> Result<String, Error> {
    run_fabric(app, "--listcontexts".to_string())
}

#[tauri::command]
pub async fn wipe_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--wipecontext={}", context))
}

#[tauri::command]
pub async fn print_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--printcontext={}", context))
}

#[tauri::command]
pub async fn get_contexts_dir(app: AppHandle) -> Result<PathBuf, Error> {
    // Get the config directory path
    let mut env_path: std::path::PathBuf = app.path().home_dir().map_err(|_e| {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        ))
    })?;

    env_path.push(".config");
    env_path.push("fabric");
    env_path.push("contexts");

    println!("contexts path: {}", env_path.display());

    Ok(env_path)
}

#[tauri::command]
pub async fn create_context_file(app: AppHandle, title: String) -> Result<String, Error> {
    let mut context_path = get_contexts_dir(app.clone()).await?;

    // Rest of the function remains the same
    context_path.push(format!("{}.md", title));

    if context_path.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Context file already exists",
        )));
    }

    std::fs::write(&context_path, "").map_err(|e| Error::Io(e))?;

    Ok(format!("Created context file: {}", context_path.display()))
}

#[tauri::command]
pub async fn read_context_file(app: AppHandle, title: String) -> Result<String, Error> {
    let mut context_path = get_contexts_dir(app.clone()).await?;
    context_path.push(format!("{}.md", title));

    if !context_path.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Context file does not exist",
        )));
    }

    std::fs::read_to_string(&context_path).map_err(|e| Error::Io(e))
}
