use bevy::prelude::*;
use rand::{random, thread_rng, Rng};

use crate::components::items::{GroundItem, GroundItemBundle};
use crate::components::physics::{Gravitate, GravitateToPlayer, Physics};
use crate::events::items::{SpawnItemEvent, SpawnKind};
use crate::resources::items::config::AssetConfig;
use crate::resources::items::ItemStore;

pub fn spawn_item_event_handler(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnItemEvent>,
    item_store: Res<ItemStore>,
    asset_server: Res<AssetServer>,
) {
    for event in spawn_event.read() {
        match event.kind.clone() {
            SpawnKind::GroundLoot {
                item_id,
                stack_count,
                position,
            } => {
                let item = item_store.get(&item_id);
                let mut entity_commands = commands.spawn((
                    Name::from(item.name.clone()),
                    GroundItemBundle {
                        ground_item: GroundItem {
                            item_id,
                            stack_count,
                            pick_up_timeout: Timer::from_seconds(2.0, TimerMode::Once),
                        },
                        gravitate: (Gravitate::default(), GravitateToPlayer),
                        physics: Physics {
                            position,
                            velocity: Vec2 {
                                x: random::<f32>() - 0.5,
                                y: random::<f32>() - 0.5,
                            }
                            .normalize()
                                * 10.0,
                            friction: 0.8,
                        },
                    },
                ));
                let transform = Transform::from_xyz(position.x, position.y, 1.0);
                if let Some(AssetConfig { ground_item_path }) = &item.assets {
                    let scale = 2.0;
                    entity_commands.insert(SpriteBundle {
                        texture: asset_server.load(ground_item_path),
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
    }
}

pub fn spawn_items(mut spawn_event: EventWriter<SpawnItemEvent>, item_store: Res<ItemStore>) {
    for (id, _) in item_store.items.iter() {
        for _ in 0..20 {
            spawn_event.send(SpawnItemEvent {
                kind: SpawnKind::GroundLoot {
                    item_id: id.clone(),
                    stack_count: thread_rng().gen_range(1..2),
                    position: Vec2 {
                        x: (random::<f32>() - 0.5) * 800.0,
                        y: (random::<f32>() - 0.5) * 800.0,
                    },
                },
            })
        }
    }
}

pub fn tick_item_timers(mut item_query: Query<&mut GroundItem>, time: Res<Time>) {
    for mut item in item_query.iter_mut() {
        item.pick_up_timeout.tick(time.delta());
    }
}
