use crate::fabric::patterns::run_fabric;
use crate::fabric::secrets::update_secret;
use std::path::PathBuf;
use tauri::{AppHandle, Error, Manager};

#[tauri::command]
// TODO move this to a specific fabric commands file
// Issue URL: https://github.com/noamsiegel/fabric-app/issues/88
pub async fn set_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--context={}", context))
}

/// Lists all contexts in the Fabric system
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
///
/// ### Returns
///
/// * `Result<String, Error>` - Success message on completion or error if operation fails
#[tauri::command]
pub async fn list_contexts(app: AppHandle) -> Result<String, Error> {
    run_fabric(app, "--listcontexts".to_string())
}

/// Removes all markdown text within a specific context.md file
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
/// * `context` - The name of the context to wipe
///
/// ### Returns
///
/// * `Result<String, Error>` - Success message on completion or error if operation fails
#[tauri::command]
pub async fn wipe_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--wipecontext={}", context))
}

/// Prints the current context
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
/// * `context` - The name of the context to print
///
/// ### Returns
///
/// * `Result<String, Error>` - Success message on completion or error if operation fails
#[tauri::command]
pub async fn print_context(app: AppHandle, context: String) -> Result<String, Error> {
    run_fabric(app, format!("--printcontext={}", context))
}

/// Gets the directory path for the Fabric contexts
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
///
/// ### Returns
///
/// * `Result<PathBuf, Error>` - Path to the Fabric contexts directory or error if operation fails
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

    Ok(env_path)
}

/// Creates a new context.md file in the Fabric contexts directory
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
/// * `title` - The name of the context to create
///
/// ### Returns
///
/// * `Result<String, Error>` - Success message on completion or error if operation fails
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

/// Reads the content of a specific context.md file
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
/// * `title` - The name of the context to read
///
/// ### Returns
///
/// * `Result<String, Error>` - Content of the context.md file or error if operation fails
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

/// Saves the content to a specific context.md file
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
/// * `title` - The name of the context to save to
/// * `content` - The content to save to the context.md file
///
/// ### Returns
///
/// * `Result<String, Error>` - Success message on completion or error if operation fails
#[tauri::command]
pub async fn save_context_file(
    app: AppHandle,
    title: String,
    content: String,
) -> Result<String, Error> {
    let mut context_path = get_contexts_dir(app.clone()).await?;
    context_path.push(format!("{}.md", title));

    if !context_path.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Context file does not exist",
        )));
    }

    std::fs::write(&context_path, content).map_err(|e| Error::Io(e))?;
    Ok("Context saved successfully".to_string())
}

/// Deletes a specific context.md file
///
/// ### Arguments
///
/// * `app` - The Tauri application handle for accessing app-wide state
/// * `title` - The name of the context to delete
///
/// ### Returns
///
/// * `Result<String, Error>` - Success message on completion or error if operation fails
#[tauri::command]
pub async fn delete_context_file(app: AppHandle, title: String) -> Result<String, Error> {
    let mut context_path = get_contexts_dir(app.clone()).await?;
    context_path.push(format!("{}.md", title));

    if !context_path.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Context file does not exist",
        )));
    }

    std::fs::remove_file(&context_path).map_err(|e| Error::Io(e))?;
    Ok(format!("Context file '{}' deleted successfully", title))
}

#[tauri::command]
pub async fn set_current_context(app: AppHandle, context: String) -> Result<String, Error> {
    // Update the CURRENT_CONTEXT in .env file
    update_secret(app.clone(), "CURRENT_CONTEXT".to_string(), context.clone())
        .await
        .map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    Ok("Current context set successfully".to_string())
}
