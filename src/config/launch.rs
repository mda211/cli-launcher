use crate::minecraft::metadata::Features;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LaunchConfig {
    pub features: Features,
}
