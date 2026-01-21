mod minecraft;

use minecraft::manifest::{VersionManifest, fetch_manifest, get_version_info};
use minecraft::metadata::Metadata;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest: VersionManifest = fetch_manifest().await?;

    let mut version_url = String::new();

    let version_id = "1.21.11";
    if let Some(version) = get_version_info(&manifest, version_id) {
        println!("Found metadata for version {}: {}", version_id, version.url);
        version_url = version.url.clone();
    } else {
        println!("Version {} not found in manifest.", version_id);
    }

    let metadata = Metadata::load(&version_url).await?;
    println!("{metadata:#?}");

    Ok(())
}
