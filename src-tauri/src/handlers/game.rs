use std::fs;

use itertools::Itertools;
use serde_json::json;
use tap::Pipe;
use tauri::{window, Manager};

use crate::{
    conf::TeamConfig,
    inline_async,
    models::{game::GameState, matchup::Matchup, schedule::Schedule},
    paths::get_team_config_path,
    sql_args,
    util::PresentError,
    POOL,
};

pub struct GameHandlers;

impl GameHandlers {
    pub fn restart_game(window: &window::Window) {
        let pool = POOL.get().unwrap();

        if let Ok(TeamConfig { teams }) =
            fs::read_to_string(get_team_config_path(&window.app_handle()))
                .present_err()
                .unwrap()
                .pipe(|s| toml::from_str::<TeamConfig>(&s))
        {
            pool.exec("DELETE FROM matchups; DELETE FROM schedules; DELETE FROM teams;");

            for team in teams {
                pool.exec_with(
                    "INSERT INTO teams (name, skill) VALUES ($1, $2);",
                    sql_args![team.name, team.skill],
                );
            }

            inline_async! {
                Schedule::create_round_robin(2023).await;
                GameState::set_week(1).await;
            }

            window.emit("game_did_restart", json!({})).unwrap();
        }
    }

    pub async fn next_week() {
        let game = GameState::get().await;

        // first, check that we even have a week to advance to

        let final_wk = Schedule::get_by_year(&game.year)
            .await
            .unwrap()
            .matchups
            .into_iter()
            .fold(0 as i32, |prev_max, matchup| matchup.wk_no.max(prev_max));

        if game.wk_no > final_wk {
            return;
        }

        // then compute all matches for this week

        let matchups_for_this_wk = Schedule::get_by_year(&game.year)
            .await
            .unwrap()
            .matchups
            .into_iter()
            .filter(|matchup| matchup.wk_no == game.wk_no)
            .collect_vec();

        for matchup in matchups_for_this_wk {
            matchup.compute_scores();
        }

        // lastly, write the new week to the control table and refetch the new game state
        GameState::set_week(game.wk_no + 1).await;
    }
}
