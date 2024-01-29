use sqlx::{
    sqlite::{SqliteArguments, SqliteConnectOptions, SqliteQueryResult, SqliteRow},
    FromRow, SqlitePool,
};
use std::ops::Deref;
use tauri::App;

use crate::{models::game::GameState, sql_args};

pub const SCHEMA_VER: i32 = 16;

#[derive(Debug)]
pub struct SqlitePoolWrapper(SqlitePool);

impl SqlitePoolWrapper {
    pub fn exec(&self, sql: &str) -> SqliteQueryResult {
        tokio::task::block_in_place(|| {
            tauri::async_runtime::block_on(async {
                sqlx::query(sql).execute(self.deref()).await.unwrap()
            })
        })
    }

    pub fn exec_with(&self, sql: &str, args: SqliteArguments) {
        tokio::task::block_in_place(|| {
            tauri::async_runtime::block_on(async {
                sqlx::query_with(sql, args)
                    .execute(self.deref())
                    .await
                    .unwrap()
            });
        })
    }

    pub async fn query<'lt, Output>(&self, sql: &'lt str) -> Vec<Output>
    where
        Output: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        sqlx::query_as(sql).fetch_all(&self.0).await.unwrap()
    }

    pub async fn query_with<'lt, Output>(
        &self, sql: &'lt str, args: SqliteArguments<'_>,
    ) -> Vec<Output>
    where
        Output: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
    {
        sqlx::query_as_with(sql, args)
            .fetch_all(&self.0)
            .await
            .unwrap()
    }
}

impl Deref for SqlitePoolWrapper {
    type Target = SqlitePool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn establish_connection(app: &App) -> SqlitePoolWrapper {
    std::fs::create_dir_all(app.path_resolver().app_local_data_dir().unwrap()).unwrap();
    let db_path = format!(
        "{}/db.sqlite",
        app.path_resolver()
            .app_local_data_dir()
            .unwrap()
            .to_str()
            .unwrap()
    );

    let opts = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts).await.unwrap();

    let ctrl_record: Option<GameState> = sqlx::query_as("SELECT * FROM ctrl;")
        .fetch_one(&pool)
        .await
        .ok();

    if ctrl_record.is_none() || ctrl_record.unwrap().schema_ver < SCHEMA_VER {
        sqlx::query(
            r#"
            PRAGMA writable_schema = 1;
            DELETE FROM sqlite_master WHERE type IN ('table', 'index', 'trigger');
            PRAGMA writable_schema = 0;
        "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query_with(include_str!("../creates.sql"), sql_args![SCHEMA_VER])
            .execute(&pool)
            .await
            .unwrap();
    }

    return SqlitePoolWrapper(pool);
}

pub struct PoolWrapper(SqlitePool);

impl From<SqlitePool> for PoolWrapper {
    fn from(value: SqlitePool) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for PoolWrapper {
    type Target = SqlitePool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
