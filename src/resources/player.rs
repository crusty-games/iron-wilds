use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerCombat {
    pub item_use_timer: Timer,
}
