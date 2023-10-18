use std::ops::Range;

use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use rand::{thread_rng, Rng};

use super::physics::Physics;

#[derive(Reflect, Component, InspectorOptions, Default)]
#[reflect(Component, InspectorOptions)]
pub struct Mob {
    pub mob_id: String,
}

#[derive(Reflect, Component, InspectorOptions, Default)]
#[reflect(Component, InspectorOptions)]
pub struct Life {
    pub max_health: f32,
    pub health: f32,
}

impl Life {
    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }
}

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct RandomWalk {
    pub active: bool,
    pub timer: Timer,
    pub target_position: Vec2,
    pub state: RandomWalkState,
    pub speed: f32,
    #[reflect(ignore)]
    pub idle_secs: Range<f32>,
    pub trigger_radius: f32,
    pub walk_radius: f32,
}

impl Default for RandomWalk {
    fn default() -> Self {
        let mut def = Self {
            active: true,
            timer: Default::default(),
            target_position: Vec2::ZERO,
            state: RandomWalkState::Idling,
            speed: 1.0,
            idle_secs: 1.0..3.0,
            trigger_radius: 50.0,
            walk_radius: 100.0,
        };
        def.idle();
        def
    }
}

impl RandomWalk {
    pub fn idle(&mut self) {
        let secs = thread_rng().gen_range(self.idle_secs.to_owned());
        self.timer = Timer::from_seconds(secs, TimerMode::Once);
        self.state = RandomWalkState::Idling;
    }

    pub fn walk(&mut self, to: Vec2) {
        self.target_position = to;
        self.state = RandomWalkState::Walking;
    }
}

#[derive(Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum RandomWalkState {
    Idling,
    Walking,
}

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub life: Life,
    pub physics: Physics,
}
