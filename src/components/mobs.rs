use bevy::prelude::*;

#[derive(Component)]
pub struct Mob {
    pub mob_id: String,
}

#[derive(Component)]
pub struct Life {
    pub max_health: f32,
    pub health: f32,
}

#[derive(Component)]
pub struct RandomWalk {
    pub timer: timer,
    pub target_position: Vec2,
    pub state: RandomWalkState,
}

pub enum RandomWalkState {
    Idling,
    Walking,
}
