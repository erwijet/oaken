use std::{collections::HashSet, ops::Deref};

use ::futures::future::join_all;
use futures::task::ArcWake;
use itertools::{iproduct, Itertools};
use serde::{Deserialize, Serialize};
use specta::Type;
use tap::Pipe;

use crate::{sql_args, util::LastInsertRowId, POOL};

use super::{matchup::Matchup, team::Team};

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Schedule {
    pub id: i32,
    pub year: i32,
    pub matchups: Vec<Matchup>,
}

#[derive(sqlx::FromRow)]
pub struct ScheduleRow {
    pub id: i32,
    pub year: i32,
}

impl Schedule {
    pub async fn create_empty(year: i32) -> Self {
        let pool = POOL.get().unwrap();
        let row: ScheduleRow = pool
            .query_with(
                r#"
            INSERT INTO schedules (year) VALUES ($1);
            SELECT * FROM schedules WHERE id = last_insert_rowid();
            "#,
                sql_args![year],
            )
            .await
            .into_iter()
            .nth(0)
            .unwrap();

        Self {
            year: row.year,
            id: row.id,
            matchups: vec![],
        }
    }

    pub async fn get_all() -> Vec<Self> {
        let pool = POOL.get().unwrap();
        let rows: Vec<ScheduleRow> = pool.query("SELECT * FROM schedules;").await;

        join_all(rows.into_iter().map(|ScheduleRow { id, year }| async move {
            let matchups: Vec<Matchup> = pool
                .query_with("SELECT * FROM matchups WHERE schedule_id = $1;", sql_args![id])
                .await;

            Schedule { matchups, year, id }
        }))
        .await
    }

    pub async fn get_by_year(year: &i32) -> Option<Self> {
        let pool = POOL.get().unwrap();
        let ScheduleRow { id, year } = pool
            .query_with("SELECT * FROM schedules WHERE year = $1", sql_args![year])
            .await
            .into_iter()
            .nth(0)?;

        let matchups: Vec<Matchup> = pool
            .query_with(
                r#"SELECT * FROM matchups WHERE schedule_id = $1;"#,
                sql_args![id],
            )
            .await;

        Some(Schedule { id, year, matchups })
    }

    pub async fn create_round_robin(year: i32) -> Self {
        let team_ids: Vec<i32> = Team::get_all().await.iter().map(|team| team.id).collect();
        let num_teams = team_ids.len();

        if num_teams % 2 != 0 {
            panic!("team count is not even");
        }

        let mut schedule = Schedule::create_empty(year).await;

        let wks = Scheduler::round_robin(team_ids);

        for wk in wks {
            for ScheduledMatch { home_id, away_id } in wk.matches {
                let matchup = Matchup::create(home_id, away_id, wk.wk_no, 0, schedule.id).await;
                schedule.matchups.push(matchup);
            }
        }

        return schedule;
    }
}

//

struct ScheduledWeek {
    wk_no: i32,
    matches: Vec<ScheduledMatch>,
}

struct ScheduledMatch {
    home_id: i32,
    away_id: i32,
}

struct Scheduler;

impl Scheduler {
    fn round_robin(players: Vec<i32>) -> Vec<ScheduledWeek> {
        let home_wks = Scheduler::schedule_weeks(players.clone(), vec![]);
        let away_wks = Scheduler::schedule_weeks(players.clone(), vec![])
            .into_iter()
            .map(|wk| ScheduledWeek {
                wk_no: wk.wk_no + home_wks.len() as i32,
                matches: wk
                    .matches
                    .into_iter()
                    .map(|scheduled_match| ScheduledMatch {
                        away_id: scheduled_match.home_id,
                        home_id: scheduled_match.away_id,
                    })
                    .collect_vec(),
            })
            .collect_vec();

        home_wks.into_iter().chain(away_wks).collect()
    }

    fn schedule_weeks(mut teams: Vec<i32>, mut wks: Vec<ScheduledWeek>) -> Vec<ScheduledWeek> {
        if teams.len() % 2 != 0 {
            return vec![];
        }

        let mut matches = Vec::<ScheduledMatch>::new();

        for n in 0..(teams.len() / 2) {
            matches.push(ScheduledMatch {
                home_id: teams[teams.len() - n - 1],
                away_id: teams[n],
            });
        }

        wks.push(ScheduledWeek {
            wk_no: wks.len() as i32 + 1,
            matches,
        });

        if wks.len() == teams.len() - 1 {
            return wks;
        }

        teams[1..].rotate_right(1);

        Scheduler::schedule_weeks(teams, wks)
    }
}
