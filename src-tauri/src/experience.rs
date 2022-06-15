use std::fs;

use serde;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Skill {
    Rust,
    JS,
    Both,
    None,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    skill: Skill,
}

fn get_storage_file_path() -> String {
    format!(
        "{}/{}",
        tauri::api::path::data_dir().unwrap().display(),
        "taurist.json"
    )
}

#[tauri::command]
pub fn get_config() -> Result<Config, String> {
    match fs::read(get_storage_file_path()) {
        Ok(config) => Ok(serde_json::from_slice(&config).unwrap()),
        Err(_e) => Err("No config yet".to_string()),
    }
}

#[tauri::command]
pub fn set_config(config: Config) -> Result<Config, String> {
    fs::write(get_storage_file_path(), serde_json::to_vec(&config).unwrap());
    get_config()
}

