use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnMob {
    pub mob_id: String,
    pub position: Vec2,
}
