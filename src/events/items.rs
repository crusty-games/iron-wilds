use bevy::prelude::*;

#[derive(Clone)]
pub enum SpawnKind {
    GroundLoot {
        item_id: String,
        stack_count: usize,
        position: Vec2,
    },
}

#[derive(Event)]
pub struct SpawnItemEvent {
    pub kind: SpawnKind,
}
