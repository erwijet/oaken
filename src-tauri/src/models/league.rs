use futures::future::join_all;
use serde::{Deserialize, Serialize};
use tap::Pipe;

use crate::{shared::pool::get_pool, sql_args};

use super::tier::Tier;

#[derive(Clone, Debug, Serialize, Deserialize, specta::Type, sqlx::FromRow)]
pub struct League {
    pub id: i32,
    pub name: String,
    pub abbr: String,
}

impl League {
    pub async fn create(name: String, abbr: String) -> Self {
        get_pool()
            .query_with(
                r#"
            INSERT INTO leagues (name, abbr) VALUES ($1, $2);
            SELECT * FROM leagues WHERE id = last_insert_rowid();
        "#,
                sql_args![&name, &abbr],
            )
            .await
            .into_iter()
            .nth(0)
            .unwrap()
    }

    pub async fn get(id: &i32) -> League {
        get_pool()
            .query_with("SELECT * FROM leagues WHERE id = $1", sql_args![id])
            .await
            .into_iter()
            .nth(0)
            .expect(&format!("failed to look up league by id: {id}"))
    }

    pub async fn get_by_name(name: String) -> League {
        get_pool()
            .query_with("SELECT * FROM leagues WHERE name = $1", sql_args![&name])
            .await
            .into_iter()
            .nth(0)
            .expect(&format!("failed to find league by name: {name}"))
    }

    pub async fn get_all() -> Vec<League> {
        get_pool().query("SELECT * FROM leagues;").await
    }

    pub async fn get_tiers(&self) -> Vec<Tier> {
        get_pool()
            .query_with(
                "SELECT * FROM tiers WHERE league_id = $1;",
                sql_args![self.id],
            )
            .await as Vec<Tier>
    }
}

#[derive(Serialize, specta::Type)]
pub struct LeagueInfo {
    pub id: i32,
    pub name: String,
    pub abbr: String,
    pub tiers: Vec<Tier>,
}

impl LeagueInfo {
    pub async fn get(league_id: i32) -> LeagueInfo {
        let league = League::get(&league_id).await;
        let tiers = league.get_tiers().await;
        let League { id, name, abbr } = league;

        Self {
            id,
            name,
            abbr,
            tiers,
        }
    }

    pub async fn get_all() -> Vec<Self> {
        League::get_all()
            .await
            .into_iter()
            .map(|league| async move { Self::get(league.id).await })
            .pipe(|it| join_all(it))
            .await
    }
}
