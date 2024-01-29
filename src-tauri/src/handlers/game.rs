use std::{fs, path::Path};

use futures::future::join_all;
use itertools::Itertools;
use rand::{Rng};
use tap::Pipe;


use crate::{
    conf::{LeagueConfig, LeagueConfigItem, TeamConfig, TeamConfigItem, TierConfigItem},
    models::{game::GameState, league::League, schedule::Schedule, team::Team, tier::Tier},
    paths::{get_leagues_config_path, get_team_config_path},
    shared::pool::get_pool,
    util::{PresentError},
};

pub struct GameHandlers;

impl GameHandlers {
    pub async fn restart_game() {
        let pool = get_pool();

        pool.exec(
            "
                    DELETE FROM matchups;
                    DELETE FROM schedules;
                    DELETE FROM teams;
                    DELETE from tiers;
                    DELETE from leagues;",
        );

        let league_config_path = get_leagues_config_path();

        let LeagueConfig { leagues, tiers } = fs::read_to_string(league_config_path)
            .present_err()
            .unwrap()
            .pipe(|s| toml::from_str(&s).present_err().unwrap());

        for LeagueConfigItem { name, abbr } in leagues {
            League::create(name, abbr).await;
        }

        let mut rank = 1;
        for TierConfigItem { name, league } in tiers {
            Tier::create(name, rank, League::get_by_name(league).await.id).await;
            rank += 1;

            if rank > 4 {
                rank = 1;
            }
        }

        let team_config_path = get_team_config_path();

        if !team_config_path.exists() {
            async fn generate_team_config(each: League) -> Vec<TeamConfigItem> {
                let skill = rand::thread_rng().gen_range(1..=100);
                let tiers = each.get_tiers().await;

                tiers
                    .iter()
                    .flat_map(|tier| {
                        (b'a'..(b'a' + 16))
                            .map(|chr| (chr as char).to_string())
                            .map(|chr| TeamConfigItem {
                                name: format!(
                                    "{}{} {}",
                                    each.abbr,
                                    tier.rank,
                                    chr.clone().to_ascii_uppercase()
                                ),
                                tier: tier.name.clone(),
                                league: each.name.clone(),
                                skill,
                            })
                            .collect_vec()
                    })
                    .collect_vec()
            }

            let config = League::get_all()
                .await
                .into_iter()
                .map(|each| generate_team_config(each))
                .pipe(|it| join_all(it))
                .await
                .into_iter()
                .flat_map(|vals| vals)
                .collect_vec()
                .pipe(|teams| TeamConfig { teams });

            fs::write::<&Path, String>(&team_config_path.as_path(), config.try_into().unwrap())
                .unwrap()
        }

        if let Ok(TeamConfig { teams }) = fs::read_to_string(team_config_path)
            .present_err()
            .unwrap()
            .pipe(|s| toml::from_str::<TeamConfig>(&s))
        {
            for team in teams {
                let league = League::get_by_name(team.league).await;
                Team::create(
                    team.name,
                    team.skill,
                    Tier::get_by_name(team.tier, league.id).await.id,
                    league.id,
                )
                .await;
            }

            for league in League::get_all().await {
                for tier in league.get_tiers().await {
                    Schedule::create_round_robin(league.id, tier.id, 2023).await;
                }
            }

            GameState::set_week(1).await;
        }
    }

    pub async fn next_week() {
        let game = GameState::get().await;

        // first, check that we even have a week to advance to

        let final_wk = Schedule::get_all_by_year(&game.year)
            .await
            .into_iter()
            .flat_map(|schedule| schedule.matchups)
            .fold(0 as i32, |prev_max, matchup| matchup.wk_no.max(prev_max));

        if game.wk_no > final_wk {
            return;
        }

        // then compute all matches for this week

        let matchups_for_this_wk = Schedule::get_all_by_year(&game.year)
            .await
            .into_iter()
            .flat_map(|schedule| schedule.matchups)
            .filter(|matchup| matchup.wk_no == game.wk_no)
            .collect_vec();

        for matchup in matchups_for_this_wk {
            matchup.compute_scores();
        }

        // lastly, write the new week to the control table and refetch the new game state
        GameState::set_week(game.wk_no + 1).await;
    }
}
