use crate::systems::setup::{say_hello, spawn_camera};
use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
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
            .add_systems(Startup, spawn_camera);
    }
}