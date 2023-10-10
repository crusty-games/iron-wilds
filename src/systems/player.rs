use crate::components::physics::Physics;
use crate::components::player::{Player, PrimaryPlayer};

use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scale = 3.0;
    commands.spawn((
        PrimaryPlayer,
        Player {
            movement_speed: 3.0,
            item_pick_up_radius: 20.0,
        },
        Physics {
            velocity: Vec2 { x: 0.0, y: 10.0 },
            ..default()
        },
        SpriteBundle {
            texture: asset_server.load("test/player.png"),
            transform: Transform::from_scale(Vec3 {
                x: scale,
                y: scale,
                z: scale,
            }),
            ..default()
        },
    ));
}
