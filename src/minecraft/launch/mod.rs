use crate::minecraft::launch::download::download_files;
use crate::minecraft::launch::library::resolve_libraries;
use crate::minecraft::metadata::{Environment, Features};
use crate::minecraft::{launch::arguments::send_arguments, metadata::Metadata};

use std::path::Path;

pub mod arguments;
pub mod download;
pub mod library;

pub async fn construct_arguments(
    metadata: &Metadata,
    environment: &Environment,
    features: &Features,
) {
    let path: &Path = Path::new("assets/libraries");

    let resolved_args = send_arguments(&metadata.arguments, environment, features);
    let download_tasks = resolve_libraries(&metadata.libraries, path);

    download_files(download_tasks).await.unwrap();

    println!("Game args:");
    for (i, arg) in resolved_args.game.iter().enumerate() {
        println!("{:>3}: {}", i, arg);
    }

    println!("\nJVM args:");
    for (i, arg) in resolved_args.jvm.iter().enumerate() {
        println!("{:>3}: {}", i, arg);
    }

    println!("Libraries downloaded.");
}
