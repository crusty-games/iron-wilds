use bevy::prelude::*;

#[derive(Component)]
pub struct PrimaryPlayer;

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub item_pick_up_radius: f32,
}
