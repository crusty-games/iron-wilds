use crate::components::physics::Physics;
use crate::components::player::Player;
use crate::resources::physics::PhysicsTimer;
use bevy::prelude::*;

pub fn move_player(
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
