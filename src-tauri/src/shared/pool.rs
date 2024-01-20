use tauri::App;
use tokio::sync::OnceCell;

use super::sql::{establish_connection, SqlitePoolWrapper};

static POOL: OnceCell<SqlitePoolWrapper> = OnceCell::const_new();

pub fn get_pool() -> &'static SqlitePoolWrapper {
    POOL.get().unwrap()
}

pub async fn init_pool(app: &App) -> &'static SqlitePoolWrapper {
    POOL.get_or_init(|| establish_connection(app)).await
}
