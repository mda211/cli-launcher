#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VersionManifest {
    versions: Vec<ManifestVersion>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ManifestVersion {
    id: String,
    r#type: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version_id = "1.21.11"; // Change this to the desired version ID

    println!("Fetching version manifest...");
    let manifest: VersionManifest = fetch_manifest().await?;

    let url = manifest
        .versions
        .iter()
        .find(|v| v.id == version_id)
        .map(|v| &v.url);

    let url = match url {
        Some(url) => url,
        None => {
            println!("Version {} not found in manifest.", version_id);
            return Ok(());
        }
    };

    let metadata = fetch_version_metadata(url).await?;
    print_version_summary(&metadata);


    Ok(())
}

fn print_version_summary(meta: &VersionMetadata) {
    println!("Found Version {}", meta.id);
    println!("----------------------");

    println!(
        "Java: {} (major {})",
        meta.java_version.component,
        meta.java_version.major_version
    );

    println!(
        "Client jar: {} ({} bytes)",
        meta.downloads.client.url,
        meta.downloads.client.size
    );

    match &meta.downloads.server {
        Some(server) => println!("Server jar: {} ({} bytes)", server.url, server.size),
        None => println!("Server jar: none"),
    }

    println!(
        "Assets: {} ({})",
        meta.asset_index.id,
        meta.asset_index.url
    );

    println!("Libraries: {}", meta.libraries.len());

    if let Some(args) = &meta.arguments {
        println!("JVM args: {}", args.jvm.len());
        println!("Game args: {}", args.game.len());
    } else {
        println!("Arguments: legacy (no argument object)");
    }
}


async fn fetch_manifest() -> Result<VersionManifest, Box<dyn std::error::Error>> {
    let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
    let response = reqwest::get(url).await?;
    let manifest = response.json::<VersionManifest>().await?;
    Ok(manifest)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VersionMetadata {
    id: String,
    java_version: JavaVersion,

    arguments: Option<Arguments>,
    downloads: Downloads,
    libraries: Vec<Library>,
    asset_index: AssetIndex,
    assets: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JavaVersion {
    component: String,
    major_version: u8,
}

#[derive(Deserialize, Debug)]
struct Arguments {
    #[serde(default)]
    game: Vec<Argument>,
    #[serde(default)]
    jvm: Vec<Argument>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Argument {
    Simple(String),
    Complex(ArgumentDetail),
}

impl Argument {
    fn values(&self) -> Vec<&str> {
        match self {
            Argument::Simple(s) => vec![s],
            Argument::Complex(detail) => match &detail.value {
                ArgValue::Single(s) => vec![s],
                ArgValue::Multiple(v) => v.iter().map(String::as_str).collect(),
            },
        }
    }
}

#[derive(Deserialize, Debug)]
struct ArgumentDetail {
    value: ArgValue,
    rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ArgValue {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum RuleAction {
    Allow,
    Disallow,
}

#[derive(Deserialize, Debug)]
struct Rule {
    action: RuleAction,
    os: Option<OS>,
    features: Option<Features>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum OSName {
    Windows,
    Linux,
    Osx,
}

#[derive(Deserialize, Debug)]
struct OS {
    name: Option<OSName>,
    arch: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Features {
    is_demo_user: Option<bool>,
    has_custom_resolution: Option<bool>,
    has_quick_plays_support: Option<bool>,
    is_quick_play_singleplayer: Option<bool>,
    is_quick_play_multiplayer: Option<bool>,
    is_quick_play_realms: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct Downloads {
    client: DownloadFile,
    server: Option<DownloadFile>,
}

#[derive(Deserialize, Debug)]
struct DownloadFile {
    sha1: String,
    size: u64,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Library {
    name: String,
    downloads: LibraryDownloads,
    rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Debug)]
struct LibraryDownloads {
    artifact: Option<DownloadFile>,
    classifiers: Option<std::collections::HashMap<String, DownloadFile>>,
}

#[derive(Deserialize, Debug)]
struct AssetIndex {
    id: String,
    sha1: String,
    size: u64,
    url: String,
}

async fn fetch_version_metadata(url: &str) -> Result<VersionMetadata, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let metadata = response.json::<VersionMetadata>().await?;
    Ok(metadata)
}