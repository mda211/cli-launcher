use crate::config::Config;
use crate::minecraft::launch::download::download_files;
use crate::minecraft::launch::library::resolve_libraries;
use crate::minecraft::metadata::Features;
use crate::minecraft::{launch::arguments::send_arguments, metadata::Metadata};

use std::path::Path;

pub mod arguments;
pub mod download;
pub mod environment;
pub mod library;

pub async fn construct_arguments(metadata: &Metadata, features: &Features, config: &Config) {
    let path: &Path = Path::new("assets/libraries");

    let resolved_args = send_arguments(&metadata.arguments, features);
    let download_tasks = resolve_libraries(&metadata.libraries, path);

    download_files(download_tasks).await.unwrap();
    println!("Libraries downloaded.");

    let game_args = arguments::replace_variables(resolved_args.game, config, metadata).join(" ");
    let jvm_args = arguments::replace_variables(resolved_args.jvm, config, metadata).join(" ");

    println!("Game args: {:?}", game_args);
    println!("JVM args: {:?}", jvm_args);
}
