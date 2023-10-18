use bevy::prelude::*;
use rand::random;

use crate::{
    components::{
        mobs::{Life, Mob, MobBundle, RandomWalk, RandomWalkState},
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
        let mut entity_commands = commands.spawn((
            Name::new(mob.name.clone()),
            MobBundle {
                mob: Mob {
                    mob_id: mob_id.clone(),
                },
                life: Life {
                    max_health: mob.max_health,
                    health: mob.max_health,
                },
                physics: Physics {
                    position: position.clone(),
                    friction: 0.5,
                    ..default()
                },
            },
        ));

        match &mob.movement {
            MovementConfig::None => {}
            MovementConfig::RandomWalk {
                speed,
                walk_radius,
                idle_secs,
            } => {
                entity_commands.insert(RandomWalk {
                    speed: speed.clone(),
                    walk_radius: walk_radius.clone(),
                    idle_secs: idle_secs.clone(),
                    ..default()
                });
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

pub fn tick_random_walk(mut movement_query: Query<&mut RandomWalk>, time: Res<Time>) {
    for mut random_walk in movement_query.iter_mut() {
        if matches!(random_walk.state, RandomWalkState::Idling) {
            random_walk.timer.tick(time.delta());
        }
    }
}

pub fn movement_random_walk(mut mob_query: Query<(&mut RandomWalk, &mut Physics)>) {
    for (mut random_walk, mut physics) in mob_query.iter_mut() {
        if !random_walk.active {
            return;
        }
        match random_walk.state {
            RandomWalkState::Idling => {
                if random_walk.timer.finished() {
                    let to = physics.position
                        + Vec2 {
                            x: (random::<f32>() - 0.5) * 2.0 * &random_walk.walk_radius,
                            y: (random::<f32>() - 0.5) * 2.0 * &random_walk.walk_radius,
                        };
                    random_walk.walk(to);
                }
            }
            RandomWalkState::Walking => {
                if random_walk.target_position.distance(physics.position)
                    < random_walk.trigger_radius
                {
                    random_walk.idle();
                } else {
                    let effect = (random_walk.target_position - physics.position).normalize();
                    let effect = effect * random_walk.speed;
                    physics.velocity += effect;
                }
            }
        }
    }
}
