use std::{env, error::Error};

use config::Config;
use serde::Deserialize;

use crate::shared_lib::structs::TorSvc;

#[derive(Deserialize, Debug)]
pub struct Settings {
    onion_address: String, // TODO: obtain this from stdin
    tor_svc: TorSvc,
}

impl Settings {
    pub fn state_dir(&self) -> &str {
        &self.tor_svc.state_dir
    }
    pub fn cache_dir(&self) -> &str {
        &self.tor_svc.cache_dir
    }
    pub fn get_onion_address(&self) -> &str {
        &self.onion_address
    }
    pub fn get_full_address(&self) -> String {
        String::from("Placeholder") // TODO:
    }
}

pub fn get_settings() -> Result<Settings, Box<dyn Error>> {
    let path = env::current_dir()?
        .join("config")
        .join("ClientSettings.toml");
    let settings = Config::builder()
        .add_source(config::File::from(path))
        .build()?;

    Ok(settings.try_deserialize()?)
}
