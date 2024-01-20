use futures::future::join_all;
use futures::Future;
use itertools::FoldWhile::*;
use itertools::Itertools;
use serde::Serialize;
use specta::Type;
use sqlx::{pool, FromRow};
use tap::Pipe;

use crate::{shared::pool::get_pool, sql_args};

use super::league::League;
use super::matchup::Matchup;
use super::tier::Tier;

#[derive(Serialize, Type, Clone, Debug, FromRow)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub skill: i32,
    pub tier_id: i32,
}

impl Team {
    pub async fn create(name: String, skill: i32, tier_id: i32, league_id: i32) -> Self {
        get_pool()
            .query_with(
                "
                INSERT INTO teams (name, skill, tier_id, league_id) VALUES ($1, $2, $3, $4);
                SELECT * FROM teams WHERE id = last_insert_rowid();
            ",
                sql_args![name, skill, tier_id, league_id],
            )
            .await
            .into_iter()
            .nth(0)
            .unwrap()
    }

    pub async fn get_all() -> Vec<Self> {
        get_pool().query("SELECT * FROM teams;").await
    }

    pub async fn get_by_division(league_id: i32, tier_id: i32) -> Vec<Team> {
        get_pool()
            .query_with(
                "SELECT * FROM teams WHERE league_id = $1 AND tier_id = $2;",
                sql_args![league_id, tier_id],
            )
            .await
    }

    pub async fn get(id: &i32) -> Self {
        get_pool()
            .query_with("SELECT * FROM teams WHERE id = $1;", sql_args![id])
            .await
            .into_iter()
            .nth(0)
            .unwrap()
    }

    pub async fn get_streak(&self, year: &i32) -> i32 {
        let mut matchups: Vec<Matchup> = get_pool()
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

#[derive(Serialize, Type)]
pub struct TeamInfo {
    pub id: i32,
    pub name: String,
    pub skill: i32,
    pub tier: Tier,
    pub league: League,
}

impl TeamInfo {
    pub async fn get(team_id: i32) -> TeamInfo {
        let Team {
            id,
            name,
            skill,
            tier_id,
        } = Team::get(&team_id).await;
        let tier = Tier::get(&tier_id).await;
        let league = League::get(&tier.league_id).await;

        Self {
            id,
            league,
            name,
            skill,
            tier,
        }
    }

    pub async fn get_all() -> Vec<Self> {
        Team::get_all()
            .await
            .into_iter()
            .map(|team| async move { Self::get(team.id).await })
            .pipe(|it| join_all(it))
            .await
    }
}
