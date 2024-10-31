use std::{path::PathBuf, sync::Mutex};

pub struct AppState {
    // fabric folder
    pub fabric_folder: Mutex<String>,
    pub fabric_dir: Mutex<PathBuf>,

    // fabric pattern
    pub default_pattern: Mutex<String>,
    pub selected_pattern: Mutex<String>,
    pub patterns: Mutex<Vec<String>>,
    // fabric state
    pub is_running: Mutex<bool>,
    // fabric LLM flags
    pub temperature: Mutex<f32>,       // -t, --temperature
    pub top_p: Mutex<f32>,             // -T, --topp
    pub presence_penalty: Mutex<f32>,  // -P, --presencepenalty
    pub frequency_penalty: Mutex<f32>, // -F, --frequencypenalty
    pub model: Mutex<String>,          // -m, --model
    pub default_model: Mutex<String>,  // -d, --defaultmodel
}
