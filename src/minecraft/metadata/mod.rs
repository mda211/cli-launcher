#![allow(dead_code)]

pub mod arguments;
pub mod library;
pub mod rule;

pub use self::rule::{Arch, Environment, Features, OS, ResolvedArguments, Rule};

use arguments::Arguments;
use library::Library;
use std::fmt;

pub struct Metadata {
    pub id: String,

    pub main_class: String,
    pub java_version: JavaVersion,

    pub arguments: Arguments,
    pub libraries: Vec<Library>,

    pub asset_index: AssetIndex,
    pub client_download: ClientDownload,
    pub compliance_level: Option<u8>,
}

#[derive(Debug)]
pub struct JavaVersion {
    pub component: String,
    pub major_version: u8,
}

#[derive(Debug)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub total_size: u64,
    pub url: String,
}

#[derive(Debug)]
pub struct ClientDownload {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

impl Metadata {
    pub async fn load(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?.text().await?;
        let json: serde_json::Value = serde_json::from_str(&response)?;

        Ok(Metadata {
            id: json["id"].as_str().unwrap_or_default().to_string(),
            java_version: JavaVersion {
                component: json["javaVersion"]["component"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                major_version: json["javaVersion"]["majorVersion"]
                    .as_u64()
                    .unwrap_or_default() as u8,
            },

            asset_index: AssetIndex {
                id: json["assetIndex"]["id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                sha1: json["assetIndex"]["sha1"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                size: json["assetIndex"]["size"].as_u64().unwrap_or_default(),
                total_size: json["assetIndex"]["totalSize"].as_u64().unwrap_or_default(),
                url: json["assetIndex"]["url"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            },

            client_download: {
                let downloads = &json["downloads"]["client"];
                ClientDownload {
                    sha1: downloads["sha1"].as_str().unwrap_or_default().to_string(),
                    size: downloads["size"].as_u64().unwrap_or_default(),
                    url: downloads["url"].as_str().unwrap_or_default().to_string(),
                }
            },

            main_class: json["mainClass"].as_str().unwrap_or_default().to_string(),
            compliance_level: json["complianceLevel"].as_u64().map(|v| v as u8),

            arguments: arguments::parse(&json)?,
            libraries: library::parse_libraries(&json)?,
        })
    }
}

impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lib_summary = library::summarize_libraries(&self.libraries);

        f.debug_struct("Metadata")
            .field("id", &self.id)
            .field("main_class", &self.main_class)
            .field(
                "java_version",
                &format!(
                    "{} (major version {})",
                    self.java_version.component, self.java_version.major_version
                ),
            )
            // .field("arguments", &self.arguments)
            .field(
                "libraries",
                &format!("{} ({})", self.libraries.len(), lib_summary),
            )
            .field("client_downloads", &self.client_download)
            .field("asset_index", &self.asset_index)
            .field("compliance_level", &self.compliance_level.unwrap())
            .finish()
    }
}
