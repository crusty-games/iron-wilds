use bevy::prelude::*;

use crate::resources::player::PlayerCombat;
use crate::systems::physics::PhysicsSet;
use crate::systems::player::PlayerSet;
use crate::systems::player::{follow_player, player_movement, spawn_player, tick_player_combat};

pub struct IronWildsPlayerPlugin;
impl Plugin for IronWildsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerCombat>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, tick_player_combat)
            .add_systems(
                Update,
                (player_movement, follow_player)
                    .chain()
                    .before(PhysicsSet::Computation)
                    .in_set(PlayerSet::Movement),
            );
    }
}
