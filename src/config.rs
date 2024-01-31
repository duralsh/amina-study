// config.rs

use serde::{Deserialize, de::DeserializeOwned};
use std::fs;
use eyre::Result;

#[derive(Deserialize)]
pub struct Config {
    pub provider_url: String,
    pub private_key: String,
}

pub fn read_toml_config<T: DeserializeOwned>(path: &str) -> Result<T> {
    let contents = fs::read_to_string(path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
