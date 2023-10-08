use bevy::prelude::*;

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
