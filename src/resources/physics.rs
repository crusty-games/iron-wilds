use bevy::prelude::*;

pub const PHYSICS_TICK_RATE_SECONDS: f32 = 1.0 / 20.0;

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
