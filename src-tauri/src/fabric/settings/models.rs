// pub mod fabric::secrets;
use crate::fabric::paths::get_fabric_config_dir;
use crate::fabric::secrets::get_secret;
use crate::fabric::secrets::update_secret;
use regex::Regex;
use serde::Serialize;
use std::process::Command;

#[tauri::command]
pub async fn refresh_models(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    // Get the output from fabric command
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

    // Get the config directory and create the markdown file path
    let mut config_dir = get_fabric_config_dir(app_handle.clone()).await?;
    config_dir.push("models.md");

    // Create markdown content
    let markdown_content = format!("# Available Models\n\n{}", models.join("\n"));

    // Write to file
    std::fs::write(&config_dir, markdown_content)
        .map_err(|e| format!("Failed to write models file: {}", e))?;

    Ok(models)
}

#[derive(Serialize)]
pub struct FormattedModel {
    id: i32,
    name: String,
    provider: String,
}

// TODO split into small functions

// TODO make providers pulled, not provided
#[tauri::command]
pub async fn get_models(app_handle: tauri::AppHandle) -> Result<Vec<FormattedModel>, String> {
    // Get the config directory and create the markdown file path
    let mut config_dir = get_fabric_config_dir(app_handle).await?;
    config_dir.push("models.md");

    // Read file contents
    let content = std::fs::read_to_string(&config_dir)
        .map_err(|e| format!("Failed to read models file: {}", e))?;

    let mut formatted_models = Vec::new();
    let mut current_provider = String::new();

    // Create regex once
    let model_regex =
        Regex::new(r"^\[(\d+)\]\s+(.+)$").map_err(|e| format!("Failed to create regex: {}", e))?;

    // Process each line
    for line in content.lines().skip(2) {
        // Skip header lines
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Check if line is a provider name
        if ["Anthropic", "Groq", "Gemini", "OpenAI"].contains(&trimmed) {
            current_provider = trimmed.to_string();
            continue;
        }

        // Parse model entries [ID] model-name
        if let Some(captures) = model_regex.captures(trimmed) {
            let id = captures[1]
                .parse::<i32>()
                .map_err(|e| format!("Failed to parse ID: {}", e))?;
            let name = captures[2].trim().to_string();

            formatted_models.push(FormattedModel {
                id,
                name,
                provider: current_provider.clone(),
            });
        }
    }

    Ok(formatted_models)
}

#[tauri::command]
pub async fn set_default_model(app: tauri::AppHandle, model: String) -> Result<(), String> {
    update_secret(app, "DEFAULT_MODEL".to_string(), model).await
}

#[tauri::command]
pub async fn get_default_model(app: tauri::AppHandle) -> Result<String, String> {
    get_secret(app, "DEFAULT_MODEL".to_string()).await
}
