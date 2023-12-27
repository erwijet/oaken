// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use handlers::config::ConfigHandlers;
use handlers::Handlers;
use menu::build_menu;
use model::{LedgerEntry, Match, Team};
use parking_lot::Mutex;
use rspc::Router;
use serde::Serialize;
use specta::Type;
use std::fs;
use std::{ops::Deref, sync::Arc};
use tauri::App;
use tokio::sync::OnceCell;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite, SqlitePool};

mod conf;
mod handlers;
mod menu;
mod model;
mod util;

async fn establish_connection(app: &App) -> SqlitePool {
    fs::create_dir_all(app.path_resolver().app_local_data_dir().unwrap()).unwrap();
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

    sqlx::query(include_str!("creates.sql"))
        .execute(&pool)
        .await
        .unwrap();

    return pool;
}

struct PoolWrapper(SqlitePool);

impl From<SqlitePool> for PoolWrapper {
    fn from(value: SqlitePool) -> Self {
        Self(value)
    }
}
impl Deref for PoolWrapper {
    type Target = SqlitePool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Type)]
struct AppState {
    teams: Vec<Team>,
}

impl AppState {
    fn new() -> Self {
        let teams: Vec<Team> = vec![];
        Self { teams }
    }
}

struct AppCtx {
    state: Arc<Mutex<AppState>>,
}

fn router() -> Arc<Router<AppCtx>> {
    let router: Router<AppCtx> = Router::new()
        .config(rspc::Config::new().export_ts_bindings("../src/bindings.d.ts"))
        .query("getTeams", |t| {
            t(|_ctx: AppCtx, _: ()| async move {
                let res: Vec<Team> = sqlx::query_as("SELECT * FROM teams;")
                    .fetch_all(POOL.get().unwrap())
                    .await
                    .unwrap();

                return res;
            })
        })
        .mutation("simulate", |t| {
            t(|_ctx, for_match: Match| async move {
                let teams: Vec<Team> = sqlx::query_as("SELECT * FROM teams;")
                    .fetch_all(POOL.get().unwrap())
                    .await
                    .unwrap();

                LedgerEntry::simulate(&teams, &for_match)
            })
        })
        .build();

    router.arced()
}

pub static POOL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

async fn init_pool(app: &App) -> &'static Pool<Sqlite> {
    POOL.get_or_init(|| establish_connection(app)).await
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState::new()));
    let context = tauri::generate_context!();

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(|app| {
            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async {
                    init_pool(app).await;

                    Ok(())
                })
            })
        })
        .menu(build_menu(&context))
        .on_menu_event(|evt| {
            let window = evt.window().clone();
            match evt.menu_item_id() {
                "create_config" => Handlers::handle_create_config(&window),
                "load_config" => Handlers::handle_load_config(&window),
                &_ => todo!(),
            }
        })
        .plugin(rspc::integrations::tauri::plugin(router(), move || {
            AppCtx {
                state: state.clone(),
            }
        }))
        .run(context)
        .expect("error while running application");
}
