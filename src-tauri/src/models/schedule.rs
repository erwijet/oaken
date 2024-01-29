use std::{collections::HashSet, ops::Deref};

use ::futures::future::join_all;
use itertools::{iproduct, Itertools};
use serde::{Deserialize, Serialize};
use specta::Type;
use tap::Pipe;

use crate::{shared::pool::get_pool, sql_args};

use super::{matchup::Matchup, team::Team, tier};

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct Schedule {
    pub id: i32,
    pub year: i32,
    pub tier_id: i32,
    pub league_id: i32,
    pub matchups: Vec<Matchup>,
}

#[derive(sqlx::FromRow)]
pub struct ScheduleRow {
    pub id: i32,
    pub tier_id: i32,
    pub league_id: i32,
    pub year: i32,
}

impl Schedule {
    pub async fn create_empty(year: i32, tier_id: i32, league_id: i32) -> Self {
        let pool = get_pool();
        let row: ScheduleRow = pool
            .query_with(
                r#"
            INSERT INTO schedules (year, tier_id, league_id) VALUES ($1, $2, $3);
            SELECT * FROM schedules WHERE id = last_insert_rowid();
            "#,
                sql_args![year, tier_id, league_id],
            )
            .await
            .into_iter()
            .nth(0)
            .unwrap();

        Self {
            year: row.year,
            league_id: row.league_id,
            tier_id: row.tier_id,
            id: row.id,
            matchups: vec![],
        }
    }

    pub async fn get_all() -> Vec<Self> {
        let pool = get_pool();
        let rows: Vec<ScheduleRow> = pool.query("SELECT * FROM schedules;").await;

        join_all(rows.into_iter().map(
            |ScheduleRow {
                 id,
                 year,
                 league_id,
                 tier_id,
             }| async move {
                let matchups: Vec<Matchup> = pool
                    .query_with(
                        "SELECT * FROM matchups WHERE schedule_id = $1;",
                        sql_args![id],
                    )
                    .await;

                Schedule {
                    matchups,
                    year,
                    id,
                    league_id,
                    tier_id,
                }
            },
        ))
        .await
    }

    pub async fn get_all_by_year(year: &i32) -> Vec<Self> {
        let pool = get_pool();
        let schedule_rows: Vec<ScheduleRow> = pool
            .query_with("SELECT * FROM schedules WHERE year = $1", sql_args![year])
            .await;

        schedule_rows
            .into_iter()
            .map(|row| async move {
                Schedule {
                    id: row.id,
                    league_id: row.league_id,
                    tier_id: row.tier_id,
                    year: row.year,
                    matchups: Matchup::get_all_for_schedule(row.id).await,
                }
            })
            .pipe(|it| join_all(it))
            .await
    }

    pub async fn create_round_robin(league_id: i32, tier_id: i32, year: i32) -> Self {
        let team_ids = Team::get_by_division(league_id, tier_id)
            .await
            .into_iter()
            .map(|team| team.id)
            .collect_vec();
        let num_teams = team_ids.len();

        if num_teams % 2 != 0 {
            panic!("team count is not even");
        }

        let mut schedule = Schedule::create_empty(year, tier_id, league_id).await;

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
