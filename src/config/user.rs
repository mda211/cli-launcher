use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfig {
    pub username: String,
    pub uuid: String,
    pub version: String,
}
