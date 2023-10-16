use bevy::prelude::*;

use crate::{events::mobs::SpawnMob, resources::mobs::MobStore};

pub fn spawn_all_mobs(mut spawn_event: EventWriter<SpawnMob>, mob_store: Res<MobStore>) {
    for mob in mob_store.mobs.values() {
        spawn_event.send(SpawnMob {
            mob_id: mob.id.to_owned(),
            position: Vec2::ZERO,
        })
    }
}

pub fn spawn_mobs(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnMob>,
    mob_store: Res<MobStore>,
) {
    for event in spawn_event.iter() {
        println!("Spawn mob: {}", event.mob_id);
    }
}
