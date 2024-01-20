// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures::{future::join_all, FutureExt};
use oaken::{
    conf::{LeagueConfig, TeamConfig, TeamConfigItem},
    handlers::game::GameHandlers,
    inline_async,
    menu::build_menu,
    models::{
        game::GameState,
        league::{self, League},
        matchup::Matchup,
        schedule::Schedule,
        standings::Standing,
        team::{Team, TeamInfo},
    },
    paths::{get_leagues_config_path, get_team_config_path},
    shared::pool::{get_pool, init_pool},
    sql_args,
};

use itertools::Itertools;
use rand::Rng;
use rspc::Router;
use serde::{Deserialize, Serialize};
use serde_json::json;
use specta::Type;
use std::{fs, path::Path, sync::Arc};
use tap::Pipe;

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
        .query("getTeamInfos", |t| {
            t(|_ctx: AppCtx, _: ()| async { TeamInfo::get_all().await })
        })
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
            t(|_ctx, year: i32| async move { Schedule::get_all_by_year(&year).await })
        })
        .query("getTeamMatchups", |t| {
            t(|_ctx, team_id: i32| async move { Matchup::get_with_teamid(&team_id).await })
        })
        .query("getStandings", |t| {
            t(|_ctx, year: i32| async move { Standing::get(&year).await })
        })
        .query("getMatchupsByWeek", |t| {
            t(|_ctx, args: GetMatchupsByWeekArgs| async move {
                let matchups: Vec<Matchup> = get_pool()
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

#[tokio::main]
async fn main() {
    let context = tauri::generate_context!();

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .setup(|app| {
            inline_async! {
                init_pool(app).await;
            }

            let league_config_path = get_leagues_config_path(&app.handle());

            if !league_config_path.exists() {
                fs::write::<&Path, String>(
                    &league_config_path.as_path(),
                    LeagueConfig::default().try_into().unwrap(),
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
