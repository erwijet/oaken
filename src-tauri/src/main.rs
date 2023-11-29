// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use model::{LedgerEntry, Match, Team};
use parking_lot::Mutex;
use rspc::Router;
use serde::Serialize;
use specta::Type;
use std::{collections::HashMap, ops::Deref, sync::Arc};

mod model;

#[derive(Serialize, Type)]
struct AppState {
    teams: Vec<Team>,
}

impl AppState {
    fn new() -> Self {
        let teams: Vec<Team> = vec![
            Team {
                id: 0,
                skill: 1,
                name: "Team A".into(),
            },
            Team {
                id: 1,
                skill: 7,
                name: "Team B".into(),
            },
            Team {
                id: 2,
                skill: 2,
                name: "Team C".into(),
            },
            Team {
                id: 3,
                skill: 4,
                name: "Team D".into(),
            },
            Team {
                id: 4,
                skill: 3,
                name: "Team E".into(),
            },
            Team {
                id: 5,
                skill: 3,
                name: "Team F".into(),
            },
            Team {
                id: 6,
                skill: 2,
                name: "Team G".into(),
            }
        ];

        Self { teams }
    }
}

struct AppCtx {
    state: Arc<Mutex<AppState>>,
}

impl From<&Arc<Mutex<AppState>>> for AppCtx {
    fn from(value: &Arc<Mutex<AppState>>) -> Self {
        AppCtx {
            state: value.clone(),
        }
    }
}

fn router() -> Arc<Router<AppCtx>> {
    let router: Router<AppCtx> = Router::new()
        .config(rspc::Config::new().export_ts_bindings("../src/bindings.d.ts"))
        .query("getTeams", |t| {
            t(|ctx: AppCtx, _: ()| ctx.state.lock().teams.clone())
        })
        .mutation("simulate", |t| {
            t(|ctx, for_match: Match| LedgerEntry::simulate(&ctx.state.lock().teams, &for_match))
        })
        .build();

    router.arced()
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();
    let state = Arc::new(Mutex::new(AppState::new()));

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .plugin(rspc::integrations::tauri::plugin(router(), move || {
            AppCtx::from(&state)
        }))
        .run(context)
        .expect("error while running application");
}
