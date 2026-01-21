#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VersionManifest {
    latest: LatestVersions,
    pub versions: Vec<IndexedVersion>,
}

#[derive(Deserialize, Debug)]
struct LatestVersions {
    release: String,
    snapshot: String,
}

#[derive(Deserialize, Debug)]
pub struct IndexedVersion {
    id: String,
    #[serde(rename = "type")]
    channel: Option<VersionChannel>,
    pub url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum VersionChannel {
    Release,
    Snapshot,
    #[serde(other)]
    Unknown,
}

pub async fn fetch_manifest() -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let manifest_url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    let response = reqwest::get(manifest_url).await?;
    let manifest: VersionManifest = response.json::<VersionManifest>().await?;
    Ok(manifest)
}

pub fn get_version_info<'a>(
    manifest: &'a VersionManifest,
    version_id: &str,
) -> Option<&'a IndexedVersion> {
    manifest.versions.iter().find(|v| v.id == version_id)
}
