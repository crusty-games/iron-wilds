use bevy::prelude::*;

use crate::systems::player::spawn_player;

pub struct IronWildsPlayerPlugin;
impl Plugin for IronWildsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
