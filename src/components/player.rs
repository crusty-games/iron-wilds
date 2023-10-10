use bevy::prelude::*;

#[derive(Component)]
pub struct PrimaryPlayer;

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
}
