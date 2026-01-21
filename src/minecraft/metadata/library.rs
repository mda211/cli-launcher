use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub action: String,
    pub os: Option<OperatingSystem>,
}

#[derive(Debug, Deserialize)]
pub struct OperatingSystem {
    pub name: OS,
}

#[derive(Debug, Deserialize, Clone)]
pub enum OS {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "osx")]
    MacOS,
    #[serde(rename = "linux")]
    Linux,
}

pub fn parse_libraries(json: &serde_json::Value) -> Result<Vec<Library>, serde_json::Error> {
    #[derive(Deserialize)]
    struct Libs {
        libraries: Vec<Library>,
    }

    let libs: Libs = serde_json::from_value(json.clone())?;
    Ok(libs.libraries)
}

impl Library {
    pub fn target_os(&self) -> Vec<OS> {
        self.rules
            .iter()
            .filter_map(|rule| rule.os.as_ref().map(|os| os.name.clone()))
            .collect()
    }
}

pub fn summarize_libraries(libraries: &[Library]) -> String {
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for lib in libraries {
        for os in lib.target_os() {
            let name = match os {
                OS::Windows => "windows",
                OS::MacOS => "macOS",
                OS::Linux => "linux",
            };

            *counts.entry(name).or_insert(0) += 1;
        }
    }

    let mut counts_vec: Vec<(&str, u32)> = counts.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));

    let parts: Vec<String> = counts_vec
        .into_iter()
        .map(|(os, count)| format!("{}: {}", os, count))
        .collect();

    parts.join(", ")
}
