mod directories;
mod launch;
mod user;

pub use directories::DirectoryConfig;
pub use launch::LaunchConfig;
pub use user::UserConfig;

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user: UserConfig,
    pub directories: DirectoryConfig,
    pub launch: LaunchConfig,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}
