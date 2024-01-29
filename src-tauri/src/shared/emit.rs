use serde::Serialize;
use serde_json::json;
use specta::Type;
use tap::Pipe;
use tauri::Manager;

use super::APP_HNDL;

#[derive(Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum EmitMsg {
    GameWillRestart,
    GameDidRestart,
    WeekDidAdvance,
}

pub struct Emitter;
impl Emitter {
    pub fn emit(msg: EmitMsg) {
        APP_HNDL
            .get()
            .unwrap()
            .pipe(|handle| handle.emit_all(&serde_json::to_string(&msg).unwrap(), json!({})))
            .expect("failed to emit");
    }
}
