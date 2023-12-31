use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{sql_args, POOL};

#[derive(Serialize, Deserialize, sqlx::FromRow, Type)]
pub struct GameState {
    pub schema_ver: i32,
    pub year: i32,
    pub wk_no: i32,
}

impl GameState {
    pub async fn get() -> Self {
        POOL.get()
            .unwrap()
            .query("SELECT * FROM ctrl;")
            .await
            .into_iter()
            .nth(0)
            .unwrap()
    }

    pub async fn set_week(wk: i32) {
        POOL.get().unwrap().exec_with(
            r#"
            UPDATE ctrl
            SET wk_no = $1;
        "#,
            sql_args![wk],
        )
    }
}
