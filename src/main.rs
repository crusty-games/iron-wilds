use bevy::{log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=off,wgpu_hal=off".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .add_systems(Startup, hello_world)
        .run();
}

fn hello_world() {
    println!("Welcome to Iron Wilds!");
}
