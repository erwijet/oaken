use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::util::Capitalize;

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamConfig {
    pub skill_level: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionConfig {
    pub teams: HashMap<String, TeamConfig>,
}

impl Default for SessionConfig {
    fn default() -> Self {
        let mut teams = HashMap::new();

        for skill_level in 1..10 {
            teams.insert(
                format!(
                    "Team {}",
                    random_word::gen(random_word::Lang::En).capitalize()
                ),
                TeamConfig { skill_level },
            );
        }

        Self { teams }
    }
}
