use bevy::prelude::*;
use rand::random;

use crate::{
    components::{
        mobs::{Life, Mob, MobBundle, RandomWalk},
        physics::Physics,
    },
    events::mobs::SpawnMob,
    resources::mobs::{
        config::{AssetConfig, MovementConfig},
        MobStore,
    },
};

pub fn spawn_all_mobs(mut spawn_event: EventWriter<SpawnMob>, mob_store: Res<MobStore>) {
    for mob in mob_store.mobs.values() {
        for _ in 0..10 {
            spawn_event.send(SpawnMob {
                mob_id: mob.id.to_owned(),
                position: Vec2 {
                    x: (random::<f32>() - 0.5) * 800.0,
                    y: (random::<f32>() - 0.5) * 800.0,
                },
            })
        }
    }
}

pub fn spawn_mobs(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnMob>,
    mob_store: Res<MobStore>,
    asset_server: Res<AssetServer>,
) {
    for SpawnMob { mob_id, position } in spawn_event.iter() {
        let mob = mob_store.get(&mob_id);
        let mut entity_commands = commands.spawn(MobBundle {
            mob: Mob {
                mob_id: mob_id.clone(),
            },
            life: Life {
                max_health: mob.max_health,
                health: mob.max_health,
            },
            physics: Physics {
                position: position.clone(),
                ..default()
            },
        });

        match mob.movement {
            MovementConfig::None => {}
            MovementConfig::RandomWalk { .. } => {
                entity_commands.insert(RandomWalk::default());
            }
        }

        if let Some(AssetConfig { test_path }) = &mob.assets {
            let transform = Transform::from_xyz(position.x, position.y, 0.0);
            let scale = 3.0;
            entity_commands.insert(SpriteBundle {
                texture: asset_server.load(test_path),
                transform: transform.with_scale(Vec3 {
                    x: scale,
                    y: scale,
                    z: scale,
                }),
                ..default()
            });
        }
    }
}
