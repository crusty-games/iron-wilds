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
