use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use tauri::AppHandle;

pub fn get_team_config_path(handle: &AppHandle) -> PathBuf {
    PathBuf::from_str(&format!(
        "{}/teams.toml",
        handle
            .path_resolver()
            .app_config_dir()
            .unwrap()
            .to_str()
            .unwrap()
    ))
    .unwrap()
}
