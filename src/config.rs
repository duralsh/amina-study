// config.rs

use serde::Deserialize;
use std::fs;
use eyre::Result;

#[derive(Deserialize)]
pub struct Config {
    provider_url: String,
    private_key: String,
    public_key: String,
}

impl Config {
    // Function to create a new Config instance by reading from a TOML file
    pub fn new(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    // Getter for provider_url
    pub fn provider_url(&self) -> &str {
        &self.provider_url
    }

    // Getter for private_key
    pub fn private_key(&self) -> &str {
        &self.private_key
    }

    // Getter for public_key
    pub fn public_key(&self) -> &str {
        &self.public_key
    }
}
