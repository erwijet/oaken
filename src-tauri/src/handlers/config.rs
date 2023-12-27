use std::fs;

use serde_json::json;
use tauri::{
    api::{dialog, shell},
    window, Manager,
};

use crate::{
    conf::{self, SessionConfig},
    util::PresentError,
    POOL,
};

use super::Handlers;

pub trait ConfigHandlers {
    fn handle_create_config(window: &window::Window) -> ();
    fn handle_load_config(window: &window::Window) -> ();
}

impl ConfigHandlers for Handlers {
    fn handle_create_config(window: &window::Window) -> () {
        let window = window.clone();

        dialog::FileDialogBuilder::new()
            .add_filter("Oaken Config File", &["toml"])
            .save_file(move |file| {
                if let Some(file) = file {
                    fs::write(
                        file.clone(),
                        toml::to_string(&conf::SessionConfig::default()).unwrap(),
                    )
                    .unwrap();

                    shell::open(
                        &window.app_handle().shell_scope(),
                        format!("file://{}", file.to_str().unwrap()),
                        None,
                    )
                    .unwrap();
                }
            });
    }

    fn handle_load_config(window: &window::Window) -> () {
        let parent_window = window.clone();
        let window = window.clone();

        dialog::confirm(
            Some(&parent_window),
            "Clear Session",
            "Do you wish to proceed?",
            |confirmed| {

                if !confirmed {
                    return;
                }

                dialog::FileDialogBuilder::new()
                    .add_filter("Oaken Config File", &["toml"])
                    .pick_file(|file| {
                        if let Some(file) = file {
                            if let Ok(s) = fs::read_to_string(file) {
                                if let Ok(conf) = toml::from_str::<SessionConfig>(&s).present_err() {
                                    tokio::task::block_in_place(|| {
                                            tauri::async_runtime::block_on(async move {
                                                let pool = POOL.get().unwrap();
                                                sqlx::query(r#"DELETE FROM teams;"#).execute(pool).await.unwrap();

                                                for team_key in conf.teams.keys() {
                                                    sqlx::query(&*format!("INSERT INTO teams (name, skill) VALUES (\"{}\", {});", 
                                                        team_key,
                                                        conf.teams.get(team_key).unwrap().skill_level.to_string()))
                                                    .execute(pool)
                                                    .await
                                                    .unwrap();
                                                }
                                                
                                                window.emit_all("config_did_load", json!({ })).unwrap();
                                            });
                                        });
                                }
                            }
                        }
                    });
            },
        );
    }
}
