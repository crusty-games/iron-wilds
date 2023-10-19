use bevy::prelude::*;

use crate::systems::{
    physics::PhysicsSet,
    player::{follow_player, player_movement, spawn_player, PlayerSet},
};

pub struct IronWildsPlayerPlugin;
impl Plugin for IronWildsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (player_movement, follow_player)
                .chain()
                .before(PhysicsSet::Computation)
                .in_set(PlayerSet::Movement),
        );
    }
}
