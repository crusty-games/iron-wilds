use bevy::prelude::*;

pub enum SpawnItemAs {
    GroundLoot { position: Vec2 },
}

#[derive(Event)]
pub struct SpawnItemEvent {
    pub item_id: String,
    pub spawn_as: SpawnItemAs,
}
