use std::{env, error::Error};

use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    tor_svc: TorSvc,
    max_connections: usize,
}

#[derive(Deserialize, Debug)]
pub struct TorSvc {
    state_dir: String,
    cache_dir: String,
}

impl Settings {
    pub fn state_dir(&self) -> &str {
        &self.tor_svc.state_dir
    }
    pub fn cache_dir(&self) -> &str {
        &self.tor_svc.cache_dir
    }
    pub fn get_max_connections(&self) -> usize {
        self.max_connections
    }
    pub fn get_full_address(&self) -> String {
        String::from("Placeholder")
    }
}

pub fn get_settings() -> Result<Settings, Box<dyn Error>> {
    let path = env::current_dir()?
        .join("config")
        .join("ServerSettings.toml");
    let settings = Config::builder()
        .add_source(config::File::from(path))
        .build()?;

    Ok(settings.try_deserialize()?)
}
