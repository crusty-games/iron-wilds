use bevy::{log::LogPlugin, prelude::*};
use bevy_prototype_lyon::prelude::ShapePlugin;

pub struct IronWildsSetupPlugin;
impl Plugin for IronWildsSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(DefaultPlugins.set(LogPlugin {
                filter: "info,wgpu_core=off,wgpu_hal=off".into(),
                level: bevy::log::Level::DEBUG,
            }))
            .add_plugins(ShapePlugin)
            .add_systems(Startup, say_hello)
            .add_systems(Startup, spawn_camera);
    }
}

fn say_hello() {
    println!(
        "\
▀█▀ ▒█▀▀█ ▒█▀▀▀█ ▒█▄░▒█ 
▒█░ ▒█▄▄▀ ▒█░░▒█ ▒█▒█▒█ 
▄█▄ ▒█░▒█ ▒█▄▄▄█ ▒█░░▀█ 

▒█░░▒█ ▒█▀▀▀█ ▒█▀▀█ ▒█░░░ ▒█▀▀▄ ▒█▀▀▀█ 
▒█▒█▒█ ▒█░░▒█ ▒█▄▄▀ ▒█░░░ ▒█░▒█ ░▀▀▀▄▄ 
▒█▄▀▄█ ▒█▄▄▄█ ▒█░▒█ ▒█▄▄█ ▒█▄▄▀ ▒█▄▄▄█"
    );
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
