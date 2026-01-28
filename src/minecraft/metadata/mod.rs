#![allow(dead_code)]

pub mod arguments;
pub mod library;
pub mod rule;

pub use self::rule::{Arch, Environment, Features, OS, ResolvedArguments, Rule, rules_allow};

use arguments::Arguments;
use library::Library;

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
