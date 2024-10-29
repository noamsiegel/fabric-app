pub mod settings;
pub use settings::{
    get_frequency_penalty, get_model, get_models, get_presence_penalty, get_temperature, get_top_p,
    set_default_model, set_frequency_penalty, set_model, set_presence_penalty, set_temperature,
    set_top_p,
};

pub mod patterns;
pub use patterns::{
    get_fabric_dir, get_patterns, get_patterns_git_folder, get_patterns_git_repo,
    get_selected_pattern, set_patterns_git_folder, set_selected_pattern,
};

pub mod secrets;
pub use secrets::{get_api_keys, get_base_urls, get_env_file_path, get_secret, update_secret};

pub mod run;
pub use run::{
    clipboard_contents_and_run_pattern, get_is_running, run_fabric_command,
    scrape_url_and_run_pattern, set_is_running,
};

pub mod install;
pub use install::install_fabric;
