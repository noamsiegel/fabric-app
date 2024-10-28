use std::{fs, path::PathBuf};
use tauri::Manager;

#[derive(serde::Serialize)]
pub struct Secret {
    name: String,
    secret: String,
}

#[tauri::command]
pub async fn get_env_file_path(app: tauri::AppHandle) -> Result<PathBuf, String> {
    let mut env_path: std::path::PathBuf = app
        .path()
        .home_dir()
        .map_err(|_| "Could not find home directory".to_string())?;

    env_path.push(".config");
    env_path.push("fabric");
    env_path.push(".env");

    Ok(env_path)
}

#[tauri::command]
pub async fn update_secret(
    app: tauri::AppHandle,
    key: String,
    value: String,
) -> Result<(), String> {
    let env_path = get_env_file_path(app).await?;

    // Read existing content or create empty string if file doesn't exist
    let content = fs::read_to_string(&env_path).unwrap_or_default();

    // Split content into lines
    let mut lines: Vec<String> = content.lines().map(String::from).collect();

    // Find if key already exists
    let key_prefix = format!("{}=", key);
    let line_index = lines.iter().position(|line| line.starts_with(&key_prefix));

    // Create new line
    let new_line = format!("{}={}", key, value);

    // Update or append the line
    match line_index {
        Some(index) => lines[index] = new_line,
        None => lines.push(new_line),
    }

    // Write back to file
    let new_content = lines.join("\n") + "\n";
    fs::write(&env_path, new_content).map_err(|_| "Could not write to .env file".to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_secret(app: tauri::AppHandle, key: String) -> Result<String, String> {
    let env_path = get_env_file_path(app).await?;

    // Read file content
    let content =
        fs::read_to_string(&env_path).map_err(|_| "Could not read .env file".to_string())?;

    // Find the line with the key
    let key_prefix = format!("{}=", key);
    let value = content
        .lines()
        .find(|line| line.starts_with(&key_prefix))
        .map(|line| line[key_prefix.len()..].to_string())
        .ok_or_else(|| format!("Key '{}' not found in .env file", key))?;

    Ok(value)
}

#[tauri::command]
pub async fn get_secrets(app: tauri::AppHandle) -> Result<Vec<Secret>, String> {
    let env_path = get_env_file_path(app).await?;

    // Read file content
    let content = fs::read_to_string(&env_path).unwrap_or_default();

    // Parse each line into a key-value pair
    let secrets: Vec<Secret> = content
        .lines()
        .filter(|line| !line.is_empty() && line.contains('='))
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let name = parts.next().unwrap_or_default().to_string();
            let secret = parts.next().unwrap_or_default().to_string();

            Secret { name, secret }
        })
        .collect();

    Ok(secrets)
}
