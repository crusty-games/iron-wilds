use bevy::prelude::*;

pub enum SpawnItemAs {
    GroundLoot { position: Vec2 },
}

#[derive(Event)]
pub struct SpawnItemEvent {
    pub item_id: String,
    pub spawn_as: SpawnItemAs,
}

impl SpawnItemEvent {
    pub fn new<S: AsRef<str>>(item_id: S, spawn_as: SpawnItemAs) -> Self {
        Self {
            item_id: item_id.as_ref().into(),
            spawn_as,
        }
    }
}
