mod minecraft;

use minecraft::launch::construct_arguments;
use minecraft::manifest::{VersionManifest, fetch_manifest, get_version_info};
use minecraft::metadata::Metadata;

use crate::minecraft::metadata::{Arch, Environment, Features, OS};

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

    let features = Features {
        is_demo_user: false,
        has_custom_resolution: false,
        has_quick_plays_support: false,
        is_quick_play_singleplayer: false,
        is_quick_play_multiplayer: false,
        is_quick_play_realms: false,
    };

    let environment = Environment {
        os: OS::Linux,
        arch: Arch::X64,
    };

    // println!("{metadata:#?}");
    construct_arguments(&metadata, &environment, &features);

    Ok(())
}
