#[allow(unused)]
mod config;
mod minecraft;

use config::Config;
use minecraft::auth;
use minecraft::manifest::{VersionManifest, fetch_manifest, get_version_info};
use minecraft::metadata::Metadata;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest: VersionManifest = fetch_manifest().await?;

    let config = Config::load("config.toml")?;
    let version_id: &str = &config.user.version;
    let features = &config.launch.features;

    println!("{:?}", features);

    let mut version_url = String::new();

    if let Some(version) = get_version_info(&manifest, version_id) {
        println!("Found metadata for version {}: {}", version_id, version.url);
        version_url = version.url.clone();
    } else {
        println!("Version {} not found in manifest.", version_id);
    }

    let metadata = Metadata::load(&version_url).await?;

    minecraft::launch::construct_arguments(&metadata, &features, &config).await;

    println!("------------------------------");

    let auth_result = auth::authenticate().await?;
    println!("Got authorization code: {}", auth_result.code);

    Ok(())
}
