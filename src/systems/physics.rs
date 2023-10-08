use bevy::prelude::*;
use std::ops::{Add, Mul};

use crate::components::physics::Physics;
use crate::resources::physics::PhysicsTimer;

pub fn tick_physics_timer(mut physics_timer: ResMut<PhysicsTimer>, time: Res<Time>) {
    physics_timer.main_tick.tick(time.delta());
}

pub fn compute_physics(mut physics_query: Query<&mut Physics>, physics_timer: Res<PhysicsTimer>) {
    if physics_timer.main_tick.finished() {
        for mut obj in physics_query.iter_mut() {
            obj.velocity = obj.velocity.mul(obj.friction);
            obj.position = obj.position.add(obj.velocity);
        }
    }
}

pub fn update_physics_shapes(
    mut player_query: Query<(&Physics, &mut Transform)>,
    physics_timer: Res<PhysicsTimer>,
) {
    let perc_left = physics_timer.main_tick.percent();
    for (object, mut transform) in player_query.iter_mut() {
        let lerp = object.position + (object.velocity * perc_left);
        transform.translation = Vec3 {
            x: lerp.x,
            y: lerp.y,
            z: transform.translation.z,
        }
    }
}
