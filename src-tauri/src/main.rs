// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures::FutureExt;
use handlers::game::GameHandlers;
use handlers::Handlers;
use itertools::Itertools;
use menu::build_menu;
use models::game::GameState;
use models::matchup::Matchup;
use models::schedule::Schedule;
use models::standings::Standing;
use models::team::{self, Team};
use parking_lot::Mutex;
use rspc::Router;
use serde::{Deserialize, Serialize};
use serde_json::json;
use specta::Type;
use std::path::Path;
use std::{fs, pin::pin};
use std::{ops::Deref, sync::Arc};
use tap::Pipe;
use tauri::{window, App, Manager};
use tokio::sync::OnceCell;

use sqlx::sqlite::{SqliteArguments, SqliteConnectOptions, SqliteQueryResult, SqliteRow};
use sqlx::{pool, Any, Arguments, Database, FromRow, IntoArguments, Pool, Sqlite, SqlitePool};

use crate::{conf::TeamConfig, paths::get_team_config_path, util::PresentError};

mod conf;
mod handlers;
mod menu;
mod model;
mod models;
mod paths;
mod util;

const SCHEMA_VER: i32 = 8;

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
        &self,
        sql: &'lt str,
        args: SqliteArguments<'_>,
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

async fn establish_connection(app: &App) -> SqlitePoolWrapper {
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

        sqlx::query_with(include_str!("creates.sql"), sql_args![SCHEMA_VER])
            .execute(&pool)
            .await
            .unwrap();
    }

    return SqlitePoolWrapper(pool);
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

struct AppCtx {}

#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
struct GetMatchupsByWeekArgs {
    year: i32,
    wk_no: i32,
}

fn router() -> Arc<Router<AppCtx>> {
    let router: Router<AppCtx> = Router::new()
        .config(rspc::Config::new().export_ts_bindings("../src/bindings.d.ts"))
        .query("getTeams", |t| {
            t(|_ctx: AppCtx, _: ()| async { Team::get_all().await })
        })
        .query("getGameState", |t| {
            t(|_ctx, _: ()| async { GameState::get().await })
        })
        .query("getSchedules", |t| {
            t(|_ctx, _: ()| async { Schedule::get_all().await })
        })
        .query("getSchedule", |t| {
            t(|_ctx, year: i32| async move { Schedule::get_by_year(&year).await })
        })
        .query("getTeamMatchups", |t| {
            t(|_ctx, team_id: i32| async move { Matchup::get_with_teamid(&team_id).await })
        })
        .query("getStandings", |t| {
            t(|_ctx, year: i32| async move { Standing::get(&year).await })
        })
        .query("getMatchupsByWeek", |t| {
            t(|_ctx, args: GetMatchupsByWeekArgs| async move {
                let matchups: Vec<Matchup> = POOL
                    .get()
                    .unwrap()
                    .query_with(
                        r#"
                                SELECT * FROM matchups
                                INNER JOIN schedules ON matchups.schedule_id = schedules.id
                                WHERE schedules.year = $1 AND matchups.wk_no = $2;
                            "#,
                        sql_args![args.year, args.wk_no],
                    )
                    .await;

                return matchups;
            })
        })
        .mutation("advanceWeek", |t| {
            t(|_ctx, _: ()| async { GameHandlers::next_week().await })
        })
        .build();

    router.arced()
}

pub static POOL: OnceCell<SqlitePoolWrapper> = OnceCell::const_new();

async fn init_pool(app: &App) -> &'static SqlitePoolWrapper {
    POOL.get_or_init(|| establish_connection(app)).await
}

#[tokio::main]
async fn main() {
    let context = tauri::generate_context!();

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(|app| {
            inline_async! {
                init_pool(app).await;
            }

            let team_config_path = get_team_config_path(&app.handle());

            if !team_config_path.exists() {
                fs::write::<&Path, String>(
                    team_config_path.as_path(),
                    TeamConfig::default().try_into().unwrap(),
                )
                .unwrap()
            }

            Ok(())
        })
        .on_menu_event(|evt| {
            let window = evt.window().clone();

            match evt.menu_item_id() {
                "restart_game" => GameHandlers::restart_game(&window),
                "next_week" => {
                    inline_async! {
                        GameHandlers::next_week().await;
                    }

                    window.emit("week_did_advance", json!({})).unwrap();
                }
                &_ => todo!(),
            }
        })
        .plugin(rspc::integrations::tauri::plugin(router(), move || {
            AppCtx {}
        }))
        .menu(build_menu(&context))
        .run(context)
        .expect("error while running application");
}
