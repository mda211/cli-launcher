use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user: User,
    pub directories: Directories,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub version: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directories {
    pub java: String,
    pub instance: String,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}
