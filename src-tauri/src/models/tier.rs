use serde::{Deserialize, Serialize};

use crate::{shared::pool::get_pool, sql_args};

#[derive(Clone, Debug, Serialize, Deserialize, specta::Type, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Tier {
    pub id: i32,
    pub name: String,
    pub rank: i32,
    pub league_id: i32,
}

impl Tier {
    pub async fn create(name: String, rank: i32, league_id: i32) -> Self {
        get_pool()
            .query_with(
                r#"
            INSERT INTO tiers (name, rank, league_id) VALUES ($1, $2, $3);
            SELECT * FROM tiers WHERE id = last_insert_rowid();
        "#,
                sql_args![&name, rank, league_id],
            )
            .await
            .into_iter()
            .nth(0)
            .unwrap()
    }

    pub async fn get(id: &i32) -> Tier {
        get_pool()
            .query_with("SELECT * FROM tiers WHERE id = $1", sql_args![id])
            .await
            .into_iter()
            .nth(0)
            .expect(&format!("Failed to look up tier with id: {id}"))
    }

    pub async fn get_by_name(name: String, league_id: i32) -> Tier {
        get_pool()
            .query_with(
                "SELECT * FROM tiers WHERE league_id = $1 AND name = $2;",
                sql_args![league_id, &name],
            )
            .await
            .into_iter()
            .nth(0)
            .expect(&format!(
                "Failed to find tier with league_id: {} and name: {}",
                league_id, name
            ))
    }
}
