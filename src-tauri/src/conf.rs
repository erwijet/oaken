use serde::{Deserialize, Serialize};

use crate::util::{Capitalize, PresentError};

#[derive(Serialize, Deserialize)]
pub struct TeamConfig {
    pub teams: Vec<TeamConfigItem>,
}

#[derive(Serialize, Deserialize)]
pub struct TeamConfigItem {
    pub name: String,
    pub skill: i32,
}

impl Default for TeamConfig {
    fn default() -> Self {
        let teams: Vec<TeamConfigItem> = (1..=10)
            .map(|skill| TeamConfigItem {
                name: format!(
                    "Team {}",
                    random_word::gen(random_word::Lang::En).capitalize()
                ),
                skill,
            })
            .collect();

        Self { teams }
    }
}

impl TryFrom<TeamConfig> for String {
    type Error = toml::ser::Error;
    fn try_from(value: TeamConfig) -> Result<Self, Self::Error> {
        toml::to_string_pretty(&value)
    }
}
