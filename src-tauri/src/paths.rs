use std::{path::PathBuf, str::FromStr};

use tauri::AppHandle;

static LEAGUES_CONFIG: &str = "/leagues.toml";
static TEAM_CONFIG: &str = "/teams.toml";

fn get_config_path(handle: &AppHandle, target: &'static str) -> PathBuf {
    PathBuf::from_str(&format!(
        "{}{target}",
        handle
            .path_resolver()
            .app_config_dir()
            .unwrap()
            .to_str()
            .unwrap()
    ))
    .unwrap()
}

pub fn get_team_config_path(handle: &AppHandle) -> PathBuf {
    get_config_path(handle, TEAM_CONFIG)
}

pub fn get_leagues_config_path(handle: &AppHandle) -> PathBuf {
    get_config_path(handle, LEAGUES_CONFIG)
}
