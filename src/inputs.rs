use bevy::prelude::*;

use crate::{
    physics::{compute_physics, Physics, PhysicsTimer},
    player::Player,
};

pub struct IronWildsInputsPlugin;
impl Plugin for IronWildsInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.before(compute_physics));
    }
}

fn move_player(
    mut player_query: Query<(&Player, &mut Physics)>,
    keyboard_input: Res<Input<KeyCode>>,
    physics_timer: Res<PhysicsTimer>,
) {
    if physics_timer.main_tick.finished() {
        for (player, mut object) in player_query.iter_mut() {
            let mut total_vec = Vec2::ZERO;
            if keyboard_input.pressed(KeyCode::W) {
                total_vec.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::S) {
                total_vec.y -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::D) {
                total_vec.x += 1.0;
            }
            if keyboard_input.pressed(KeyCode::A) {
                total_vec.x -= 1.0;
            }
            if total_vec.length() > 0.0 {
                total_vec = total_vec.normalize() * player.movement_speed;
                object.velocity += total_vec;
            }
        }
    }
}
