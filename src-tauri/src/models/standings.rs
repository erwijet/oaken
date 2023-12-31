use futures::future::join_all;
use serde::{Deserialize, Serialize};
use specta::Type;
use sqlx::prelude::*;

use crate::POOL;

use super::team::Team;

#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Standing {
    pub team_id: i32,
    pub team_name: String,
    pub wins: i32,
    pub losses: i32,
    pub draws: i32,
    pub points_for: i32,
    pub points_against: i32,
    pub streak: i32,
    pub win_percent: f64,
}

#[derive(Deserialize, FromRow)]
struct StandingRow {
    team_id: i32,
    team_name: String,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
}

impl Standing {
    pub async fn get(year: &i32) -> Vec<Self> {
        let pool = POOL.get().unwrap();
        let rows: Vec<StandingRow> = pool.query(r#"
            SELECT
                teams.id AS team_id,
                teams.name AS team_name,
                COUNT(CASE WHEN teams.id = matchups.home_team_id AND home_team_score > away_team_score THEN 1 END) +
                COUNT(CASE WHEN teams.id = matchups.away_team_id AND away_team_score > home_team_score THEN 1 END) AS wins,
                COUNT(CASE WHEN (teams.id = matchups.home_team_id OR teams.id = matchups.away_team_id) AND home_team_score = away_team_score THEN 1 END) AS draws,
                COUNT(CASE WHEN teams.id = matchups.home_team_id AND home_team_score < away_team_score THEN 1 END) +
                COUNT(CASE WHEN teams.id = matchups.away_team_id AND away_team_score < home_team_score THEN 1 END) AS losses,
                SUM(CASE WHEN teams.id = matchups.home_team_id THEN home_team_score ELSE away_team_score END) AS points_for,
                SUM(CASE WHEN teams.id = matchups.home_team_id THEN away_team_score ELSE home_team_score END) AS points_against
            FROM teams
            LEFT JOIN matchups
                ON teams.id = matchups.home_team_id OR teams.id = matchups.away_team_id
            GROUP BY teams.id, teams.name
            ORDER BY wins - losses DESC;
        "#).await;

        join_all(rows.into_iter().map(|row| async move {
            Standing {
                team_id: row.team_id,
                points_for: row.points_for,
                points_against: row.points_against,
                team_name: row.team_name,
                wins: row.wins,
                draws: row.draws,
                losses: row.losses,
                streak: Team::get(&row.team_id).await.get_streak(year).await,
                win_percent: ((row.wins as f64) + (0.5 * row.draws as f64))
                    / (row.wins + row.draws + row.losses) as f64,
            }
        }))
        .await
    }
}
