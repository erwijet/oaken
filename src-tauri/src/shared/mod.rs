use std::sync::OnceLock;

use tauri::AppHandle;

pub mod pool;
pub mod sql;
pub mod emit;

pub static APP_HNDL: OnceLock<AppHandle> = OnceLock::new();
