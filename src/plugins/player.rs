use bevy::prelude::*;

use crate::systems::player::{follow_player, spawn_player};

pub struct IronWildsPlayerPlugin;
impl Plugin for IronWildsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, follow_player);
    }
}
