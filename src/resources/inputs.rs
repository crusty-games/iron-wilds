use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameInputs {
    pub movement: MultiInputAxis,
}

#[derive(Default)]
pub struct MultiInputAxis {
    pub keyboard: Vec2,
    pub controller: Vec2,
}

pub fn clamp_vec2(vec: Vec2) -> Vec2 {
    if vec.x.abs() > 1.0 || vec.y.abs() > 1.0 {
        vec.normalize()
    } else {
        vec
    }
}

impl MultiInputAxis {
    pub fn combine(&self) -> Vec2 {
        let keyboard = clamp_vec2(self.keyboard);
        let controller = clamp_vec2(self.controller);
        clamp_vec2(Vec2 {
            x: keyboard.x + controller.x,
            y: keyboard.y + controller.y,
        })
    }
}
