use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes::Circle,
};
use rand::{random, thread_rng, Rng};

use crate::{
    components::{
        items::{GroundItem, GroundItemBundle},
        physics::{Gravitate, GravitateToPlayer, Physics},
    },
    events::items::{SpawnItemEvent, SpawnKind},
    game::items::store::ITEM_STORE,
};

pub fn spawn_item_event_handler(
    mut commands: Commands,
    mut spawn_item_event: EventReader<SpawnItemEvent>,
) {
    for event in spawn_item_event.iter() {
        match event.kind.clone() {
            SpawnKind::GroundLoot {
                item_id,
                stack_count,
                position,
            } => {
                let item = ITEM_STORE.get(&item_id);
                commands.spawn((
                    Name::from(item.name.clone()),
                    GroundItemBundle {
                        ground_item: GroundItem {
                            item_id,
                            stack_count,
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
                        shape: ShapeBundle {
                            path: GeometryBuilder::build_as(&Circle {
                                radius: 5.0,
                                ..default()
                            }),
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        },
                        fill: Fill::color(Color::RED),
                    },
                ));
            }
        }
    }
}

pub fn spawn_items(mut spawn_item_event: EventWriter<SpawnItemEvent>) {
    for (id, _) in ITEM_STORE.items.iter() {
        for _ in 0..10 {
            spawn_item_event.send(SpawnItemEvent {
                kind: SpawnKind::GroundLoot {
                    item_id: id.clone(),
                    stack_count: thread_rng().gen_range(1..2),
                    position: Vec2 {
                        x: (random::<f32>() - 0.5) * 500.0,
                        y: (random::<f32>() - 0.5) * 500.0,
                    },
                },
            })
        }
    }
}
