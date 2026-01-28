#![allow(dead_code)]

use crate::minecraft::metadata::{Environment, Features};
use crate::minecraft::{launch::arguments::send_arguments, metadata::Metadata};

pub mod arguments;

pub fn construct_arguments(metadata: &Metadata, environment: &Environment, features: &Features) {
    let resolved = send_arguments(&metadata.arguments, environment, features);

    println!("Game args:");
    for (i, arg) in resolved.game.iter().enumerate() {
        println!("{:>3}: {}", i, arg);
    }

    println!("\nJVM args:");
    for (i, arg) in resolved.jvm.iter().enumerate() {
        println!("{:>3}: {}", i, arg);
    }
}
