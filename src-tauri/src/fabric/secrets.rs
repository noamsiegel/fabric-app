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

    // Find if key exists
    let key_prefix = format!("{}=", key);
    if let Some(line) = lines.iter_mut().find(|line| line.starts_with(&key_prefix)) {
        // Update existing key
        *line = format!("{}={}", key, value);
    } else {
        // Key doesn't exist, append it
        lines.push(format!("{}={}", key, value));
    }

    // Write back to file
    let new_content = if lines.is_empty() {
        format!("{}={}\n", key, value)
    } else {
        lines.join("\n") + "\n"
    };

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
pub async fn get_api_keys(app: tauri::AppHandle) -> Result<Vec<Secret>, String> {
    let env_path = get_env_file_path(app).await?;

    // Read file content
    let content = fs::read_to_string(&env_path).unwrap_or_default();

    // Parse each line into a key-value pair and filter for API_KEY
    let secrets: Vec<Secret> = content
        .lines()
        .filter(|line| !line.is_empty() && line.contains('='))
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let name = parts.next().unwrap_or_default().to_string();
            let secret = parts.next().unwrap_or_default().to_string();

            Secret { name, secret }
        })
        .filter(|secret| secret.name.contains("API_KEY"))
        .collect();

    Ok(secrets)
}

#[tauri::command]
pub async fn get_base_urls(app: tauri::AppHandle) -> Result<Vec<Secret>, String> {
    let env_path = get_env_file_path(app).await?;

    // Read file content
    let content = fs::read_to_string(&env_path).unwrap_or_default();

    // Parse each line into a key-value pair and filter for API_KEY
    let secrets: Vec<Secret> = content
        .lines()
        .filter(|line| !line.is_empty() && line.contains('='))
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let name = parts.next().unwrap_or_default().to_string();
            let secret = parts.next().unwrap_or_default().to_string();

            Secret { name, secret }
        })
        .filter(|secret| secret.name.contains("BASE_URL"))
        .collect();

    Ok(secrets)
}

#[tauri::command]
pub async fn get_pattern_secrets(app: tauri::AppHandle) -> Result<Vec<Secret>, String> {
    let env_path = get_env_file_path(app).await?;

    // Read file content
    let content = fs::read_to_string(&env_path).unwrap_or_default();

    // Parse each line into a key-value pair and filter for API_KEY
    let secrets: Vec<Secret> = content
        .lines()
        .filter(|line| !line.is_empty() && line.contains('='))
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let name = parts.next().unwrap_or_default().to_string();
            let secret = parts.next().unwrap_or_default().to_string();

            Secret { name, secret }
        })
        .filter(|secret| secret.name.contains("PATTERNS"))
        .collect();

    Ok(secrets)
}

// #[tauri::command]
// pub async fn get_default_model(app: tauri::AppHandle) -> Result<Vec<Secret>, String> {
//     let env_path = get_env_file_path(app).await?;

//     // Read file content
//     let content = fs::read_to_string(&env_path).unwrap_or_default();

//     // Parse each line into a key-value pair and filter for API_KEY
//     let secrets: Vec<Secret> = content
//         .lines()
//         .filter(|line| !line.is_empty() && line.contains('='))
//         .map(|line| {
//             let mut parts = line.splitn(2, '=');
//             let name = parts.next().unwrap_or_default().to_string();
//             let secret = parts.next().unwrap_or_default().to_string();

//             Secret { name, secret }
//         })
//         .filter(|secret| secret.name.contains("DEFAULT_MODEL"))
//         .collect();

//     Ok(secrets)
// }

// #[tauri::command]
// pub async fn get_default_vendor(app: tauri::AppHandle) -> Result<Vec<Secret>, String> {
//     let env_path = get_env_file_path(app).await?;

//     // Read file content
//     let content = fs::read_to_string(&env_path).unwrap_or_default();

//     // Parse each line into a key-value pair and filter for API_KEY
//     let secrets: Vec<Secret> = content
//         .lines()
//         .filter(|line| !line.is_empty() && line.contains('='))
//         .map(|line| {
//             let mut parts = line.splitn(2, '=');
//             let name = parts.next().unwrap_or_default().to_string();
//             let secret = parts.next().unwrap_or_default().to_string();

//             Secret { name, secret }
//         })
//         .filter(|secret| secret.name.contains("DEFAULT_VENDOR"))
//         .collect();

//     Ok(secrets)
// }
