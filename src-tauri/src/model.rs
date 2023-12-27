use std::{borrow::BorrowMut, fmt::format};

use rand::Rng;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Type, Clone, Debug, sqlx::FromRow)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub skill: i32,
}

#[derive(Serialize, Deserialize, Type)]
pub struct Match {
    pub id: i32,
    pub wk_no: u16,
    pub season_id: i32,
    pub home_team_id: i32,
    pub away_team_id: i32,
}

#[derive(Serialize)]
pub struct Schedule {
    pub matches: Vec<Match>,
}

//

#[derive(Serialize)]
pub struct Ledger {
    pub entries: Vec<LedgerEntry>,
}

#[derive(Serialize, Type)]
pub struct LedgerEntry {
    pub match_id: i32,
    pub home_score: i32,
    pub away_score: i32,
}

impl LedgerEntry {
    pub fn simulate(teams: &Vec<Team>, for_match: &Match) -> Self {
        let home_team = teams
            .iter()
            .find(|team| team.id == for_match.home_team_id)
            .expect("unknown team id");
        let away_team = teams
            .iter()
            .find(|team| team.id == for_match.away_team_id)
            .expect("unknown team id");

        let mut rng = rand::thread_rng();

        let home_score = (home_team.skill + rng.gen_range(-2..5)).max(0);
        let away_score = (away_team.skill + rng.gen_range(-2..5)).max(0);

        Self {
            match_id: for_match.id,
            home_score,
            away_score,
        }
    }
}

//
