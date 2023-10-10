use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes::Circle,
};
use rand::random;

use crate::{
    components::{
        items::{GroundItem, GroundItemBundle},
        physics::Physics,
    },
    events::items::{SpawnItemAs, SpawnItemEvent},
    resources::items::ItemStore,
};

pub fn spawn_item_event_handler(
    mut commands: Commands,
    mut spawn_item_event: EventReader<SpawnItemEvent>,
    item_store: Res<ItemStore>,
) {
    for event in spawn_item_event.iter() {
        let item = item_store.get(&event.item_id);
        let mut entity_commands = commands.spawn_empty();
        entity_commands.insert(Name::from(item.name.clone()));

        match event.spawn_as {
            SpawnItemAs::GroundLoot { position } => {
                entity_commands.insert(GroundItemBundle {
                    ground_item: GroundItem::default(),
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
                });
            }
        }
    }
}

pub fn spawn_items(mut spawn_item_event: EventWriter<SpawnItemEvent>, item_store: Res<ItemStore>) {
    for (id, _) in item_store.items.iter() {
        spawn_item_event.send(SpawnItemEvent::new(
            id,
            SpawnItemAs::GroundLoot {
                position: Vec2::ZERO,
            },
        ))
    }

    spawn_item_event.send(SpawnItemEvent::new(
        "bread",
        SpawnItemAs::GroundLoot {
            position: Vec2::ZERO,
        },
    ))
}
