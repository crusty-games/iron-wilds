use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use std::ops::{Add, Mul};

pub const PHYSICS_TICK_RATE_SECONDS: f32 = 1.0 / 20.0;
pub struct IronWildsPhysicsPlugin;
impl Plugin for IronWildsPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsTimer>()
            .register_type::<Physics>()
            .add_systems(Update, tick_physics_timer)
            .add_systems(Update, compute_physics)
            .add_systems(Update, update_physics_shapes.after(compute_physics));
    }
}

#[derive(Resource)]
pub struct PhysicsTimer {
    pub main_tick: Timer,
}

impl Default for PhysicsTimer {
    fn default() -> Self {
        Self {
            main_tick: Timer::from_seconds(PHYSICS_TICK_RATE_SECONDS, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
    #[inspector(min = 0.0, max = 1.0)]
    pub friction: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            friction: 0.8,
        }
    }
}

fn tick_physics_timer(mut physics_timer: ResMut<PhysicsTimer>, time: Res<Time>) {
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

fn update_physics_shapes(
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
