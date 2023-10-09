use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

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
