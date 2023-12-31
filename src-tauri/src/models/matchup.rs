use rand::Rng;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{inline_async, sql_args, POOL};

use super::team::Team;

#[derive(Serialize, Deserialize, Type, Clone, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Matchup {
    pub id: i32,
    pub wk_no: i32,
    pub home_team_id: i32,
    pub away_team_id: i32,

    pub home_team_score: Option<i32>,
    pub away_team_score: Option<i32>,
}

impl Matchup {
    pub async fn create(
        home_team_id: i32,
        away_team_id: i32,
        wk_no: i32,
        season_id: i32,
        schedule_id: i32,
    ) -> Self {
        POOL.get()
            .unwrap()
            .query_with(
                r#"
                INSERT INTO matchups (wk_no, season_id, home_team_id, away_team_id, schedule_id)
                VALUES ($1, $2, $3, $4, $5);
                SELECT * FROM matchups WHERE id = last_insert_rowid();
            "#,
                sql_args![wk_no, season_id, home_team_id, away_team_id, schedule_id],
            )
            .await
            .into_iter()
            .nth(0)
            .unwrap()
    }

    pub async fn get_with_teamid(team_id: &i32) -> Vec<Self> {
        POOL.get()
            .unwrap()
            .query_with(
                r#"
                SELECT * FROM matchups
                WHERE home_team_id = $1 OR away_team_id = $1
            "#,
                sql_args![team_id],
            )
            .await
    }

    /// Computes the scores for this matchup refetching and returning the result once done
    pub fn compute_scores(&self) {
        let mut rng = rand::thread_rng();
        let pool = POOL.get().unwrap();

        inline_async! {{
            let home_team = Team::get(&self.home_team_id).await;
            let away_team = Team::get(&self.away_team_id).await;

            let home_team_score = (home_team.skill + rng.gen_range(-2..5)).max(0);
            let away_team_score = (away_team.skill + rng.gen_range(-2..5)).max(0);

            pool.exec_with(
                r#"
                    UPDATE matchups
                    SET home_team_score = $2, away_team_score = $3
                    WHERE id = $1;
                "#,
                sql_args![self.id, home_team_score, away_team_score],
            )
        }}
    }
}
