use bevy::prelude::*;

use super::physics::Physics;

#[derive(Component)]
pub struct Mob {
    pub mob_id: String,
}

#[derive(Component)]
pub struct Life {
    pub max_health: f32,
    pub health: f32,
}

#[derive(Component, Default)]
pub struct RandomWalk {
    pub timer: Timer,
    pub target_position: Vec2,
    pub state: RandomWalkState,
}

#[derive(Default)]
pub enum RandomWalkState {
    #[default]
    Idling,
    Walking,
}

#[derive(Bundle)]
pub struct MobBundle {
    pub mob: Mob,
    pub life: Life,
    pub physics: Physics,
}
