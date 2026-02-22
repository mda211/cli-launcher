use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfig {
    pub java: String,
    pub instance: String,
}
