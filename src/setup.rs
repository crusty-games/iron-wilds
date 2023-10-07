use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;

use crate::logic::items::store::GAME_ITEM_STORE;

pub struct IronWildsSetupPlugin;
impl Plugin for IronWildsSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_plugins(DefaultPlugins.set(LogPlugin {
                filter: "info,wgpu_core=off,wgpu_hal=off".into(),
                level: bevy::log::Level::DEBUG,
            }))
            .add_plugins(ShapePlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(Startup, say_hello)
            .add_systems(Startup, print_items.after(say_hello))
            .add_systems(Startup, spawn_camera);
    }
}

fn say_hello() {
    println!(
        "\
░▀█▀░█▀▄░█▀█░█▀█    
░░█░░█▀▄░█░█░█░█    
░▀▀▀░▀░▀░▀▀▀░▀░▀    
░█░█░▀█▀░█░░░█▀▄░█▀▀
░█▄█░░█░░█░░░█░█░▀▀█
░▀░▀░▀▀▀░▀▀▀░▀▀░░▀▀▀"
    );
}

fn print_items() {
    println!("Items loaded in store: {}", GAME_ITEM_STORE.items.len());
    for item in GAME_ITEM_STORE.items.iter() {
        println!("{}: {}", item.id, item.name);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
