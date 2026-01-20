#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VersionManifest {
    versions: Vec<VersionInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VersionInfo {
    id: String,
    r#type: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version_id = "1.21.11";

    println!("Fetching version manifest...");
    let manifest: VersionManifest = fetch_manifest().await?;

    let mut url: String = String::new();

    for version in manifest.versions.iter() {
        if version.id == version_id {
            println!("Found version: {:#?}", version);
            url = version.url.clone();
            break;
        }
    }
    
    let version_info = fetch_version_info(url);
    println!("Version info: {:#?}", version_info.await?);


    Ok(())
}

async fn fetch_manifest() -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    let response = reqwest::get(url).await?;
    let manifest = response.json::<VersionManifest>().await?;
    Ok(manifest)
}

async fn fetch_version_info(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let version_info: String = response.text().await?;
    Ok(version_info)
}