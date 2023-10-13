use bevy::prelude::*;
use std::ops::{Add, Mul};

use crate::components::physics::{Gravitate, Physics};
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

pub fn gravitate<Marker: Component, Target: Component>(
    mut physics_query: Query<(&mut Physics, &Gravitate), (With<Marker>, Without<Target>)>,
    target_query: Query<&Physics, (With<Target>, Without<Marker>)>,
    physics_timer: Res<PhysicsTimer>,
) {
    if physics_timer.main_tick.finished() {
        for (mut object, gravity) in physics_query.iter_mut() {
            for target in target_query.iter() {
                let dist = object
                    .position
                    .distance(target.position)
                    .max(gravity.min_radius);
                if dist < gravity.max_radius {
                    let pos_delta = object.position - target.position;
                    let impulse = gravity.min_radius / (dist * dist);
                    let total_effect = pos_delta * impulse;
                    object.velocity -= total_effect * gravity.strength;
                }
            }
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
