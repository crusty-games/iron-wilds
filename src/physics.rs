use bevy::prelude::*;
use std::ops::{Add, Mul};

pub const PHYSICS_TICK_RATE_SECONDS: f32 = 1.0 / 20.0;
pub struct IronWildsPhysicsPlugin;
impl Plugin for IronWildsPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsTimer>()
            .add_systems(Update, tick_physics_timer)
            .add_systems(Update, compute_physics);
    }
}

#[derive(Resource)]
pub struct PhysicsTimer {
    pub timer: Timer,
}

impl Default for PhysicsTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(PHYSICS_TICK_RATE_SECONDS, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
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
    physics_timer.timer.tick(time.delta());
}

fn compute_physics(mut physics_query: Query<&mut Physics>, physics_timer: Res<PhysicsTimer>) {
    if physics_timer.timer.finished() {
        for mut obj in physics_query.iter_mut() {
            obj.velocity = obj.velocity.mul(obj.friction);
            obj.position = obj.position.add(obj.velocity);
        }
    }
}
