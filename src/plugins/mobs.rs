use bevy::prelude::*;

use crate::{
    components::mobs::RandomWalk,
    events::mobs::SpawnMob,
    resources::mobs::MobStore,
    systems::mobs::{movement_random_walk, spawn_all_mobs, spawn_mobs, tick_random_walk},
};

pub struct IronWildsMobsPlugin;
impl Plugin for IronWildsMobsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MobStore>()
            .register_type::<RandomWalk>()
            .add_event::<SpawnMob>()
            .add_systems(Startup, spawn_all_mobs)
            .add_systems(Update, (spawn_mobs, tick_random_walk, movement_random_walk));
    }
}
