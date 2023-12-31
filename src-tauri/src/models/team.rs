use itertools::FoldWhile::*;
use itertools::Itertools;
use serde::Serialize;
use specta::Type;
use sqlx::{pool, FromRow};

use crate::{sql_args, POOL};

use super::matchup::Matchup;

#[derive(Serialize, Type, Clone, Debug, FromRow)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub skill: i32,
}

impl Team {
    pub async fn get_all() -> Vec<Self> {
        POOL.get().unwrap().query("SELECT * FROM teams;").await
    }

    pub async fn get(id: &i32) -> Self {
        POOL.get()
            .unwrap()
            .query_with("SELECT * FROM teams WHERE id = $1;", sql_args![id])
            .await
            .into_iter()
            .nth(0)
            .unwrap()
    }

    pub async fn get_streak(&self, year: &i32) -> i32 {
        let mut matchups: Vec<Matchup> = POOL
            .get()
            .unwrap()
            .query_with(
                r#"
                SELECT * FROM matchups
                JOIN schedules ON schedules.id = matchups.schedule_id
                WHERE schedules.year = $1 
                    AND (home_team_id = $2 OR away_team_id = $2)
                    AND (home_team_score IS NOT NULL AND away_team_score IS NOT NULL);"#,
                sql_args![year, self.id],
            )
            .await;

        matchups.sort_by_key(|each| each.wk_no);

        matchups
            .iter()
            .rev()
            .fold_while(0, |streak, matchup| {
                if matchup.away_team_score > matchup.home_team_score
                    && matchup.away_team_id == self.id
                {
                    return Continue(streak + 1);
                }

                if matchup.away_team_score < matchup.home_team_score
                    && matchup.home_team_id == self.id
                {
                    return Continue(streak + 1);
                }

                return Done(streak);
            })
            .into_inner()
    }
}
