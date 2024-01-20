use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TeamConfig {
    pub teams: Vec<TeamConfigItem>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamConfigItem {
    pub name: String,
    pub skill: i32,
    pub tier: String,
    pub league: String
}

impl TryFrom<TeamConfig> for String {
    type Error = toml::ser::Error;
    fn try_from(value: TeamConfig) -> Result<Self, Self::Error> {
        toml::to_string_pretty(&value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct LeagueConfig {
    pub leagues: Vec<LeagueConfigItem>,
    pub tiers: Vec<TierConfigItem>,
}

#[derive(Serialize, Deserialize)]
pub struct LeagueConfigItem {
    pub abbr: String,
    pub name: String,
}

impl From<(&str, &str)> for LeagueConfigItem {
    fn from(value: (&str, &str)) -> Self {
        LeagueConfigItem {
            abbr: value.0.to_owned(),
            name: value.1.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TierConfigItem {
    pub name: String,
    pub league: String,
}

impl Default for LeagueConfig {
    fn default() -> Self {
        let leagues: Vec<LeagueConfigItem> = [
            ("NE", "Northeast"),
            ("AT", "Atlantic"),
            ("SE", "Southeast"),
            ("GL", "Great Lakes"),
            ("SO", "South"),
            ("SW", "Southwest"),
            ("NW", "Northwest"),
            ("CT", "Central"),
        ]
        .into_iter()
        .map_into()
        .collect_vec();

        let tiers = leagues
            .iter()
            .flat_map(|league| {
                (1..=4).map(|i| TierConfigItem {
                    league: league.name.to_owned(),
                    name: format!("Tier {i}"),
                })
            })
            .collect_vec();

        Self { leagues, tiers }
    }
}

impl TryFrom<LeagueConfig> for String {
    type Error = toml::ser::Error;
    fn try_from(value: LeagueConfig) -> Result<Self, Self::Error> {
        toml::to_string_pretty(&value)
    }
}
