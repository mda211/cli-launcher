use serde::Deserialize;

use crate::minecraft::metadata::Rule;

#[derive(Debug, Deserialize)]
pub struct Library {
    pub name: String,
    pub downloads: Downloads,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub artifact: Artifact,
}

#[derive(Debug, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub url: String,
    pub sha1: String,
    pub size: Option<u64>,
}

pub fn parse_libraries(json: &serde_json::Value) -> Result<Vec<Library>, serde_json::Error> {
    #[derive(Deserialize)]
    struct Libs {
        libraries: Vec<Library>,
    }

    let libs: Libs = serde_json::from_value(json.clone())?;
    Ok(libs.libraries)
}
