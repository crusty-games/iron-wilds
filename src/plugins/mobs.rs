use bevy::prelude::*;

use crate::{
    events::mobs::SpawnMob,
    resources::mobs::MobStore,
    systems::mobs::{spawn_all_mobs, spawn_mobs},
};

pub struct IronWildsMobsPlugin;
impl Plugin for IronWildsMobsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MobStore>()
            .add_event::<SpawnMob>()
            .add_systems(Startup, spawn_all_mobs)
            .add_systems(Update, spawn_mobs);
    }
}
