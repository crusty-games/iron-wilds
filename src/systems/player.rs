use crate::components::physics::Physics;
use crate::components::player::{Player, PrimaryPlayer};
use crate::resources::inputs::GameInputs;
use crate::resources::physics::PhysicsTimer;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSet {
    Movement,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scale = 1.0 / 18.0;
    commands.spawn((
        Name::new("PrimaryPlayer"),
        PrimaryPlayer,
        Player {
            movement_speed: 0.05,
            item_pick_up_radius: 0.4,
        },
        Physics::default(),
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

pub fn player_movement(
    mut player_query: Query<(&Player, &mut Physics), With<PrimaryPlayer>>,
    game_inputs: Res<GameInputs>,
    physics_timer: Res<PhysicsTimer>,
) {
    if !physics_timer.main_tick.finished() {
        return;
    }
    for (player, mut physics) in player_query.iter_mut() {
        physics.velocity += game_inputs.movement.combine() * player.movement_speed;
    }
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PrimaryPlayer>)>,
    player_query: Query<&Transform, (With<PrimaryPlayer>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    for mut camera in camera_query.iter_mut() {
        for player in player_query.iter() {
            let translation =
                (player.translation - camera.translation) * time.delta_seconds() * 4.0;
            camera.translation += translation;
        }
    }
}
