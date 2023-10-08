use crate::systems::physics::compute_physics;
use crate::systems::player::{spawn_player, update_player_shape};
use bevy::prelude::*;

pub struct IronWildsPlayerPlugin;
impl Plugin for IronWildsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, update_player_shape.after(compute_physics));
    }
}
