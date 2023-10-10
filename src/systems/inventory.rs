use bevy::prelude::*;

use crate::components::items::GroundItem;
use crate::components::player::{Player, PrimaryPlayer};
use crate::components::{physics::Physics, storage::StorageItem};
use crate::resources::{inventory::Inventory, items::Items};

pub fn pick_up_ground_items(
    mut commands: Commands,
    player_query: Query<(&Player, &Physics), With<PrimaryPlayer>>,
    item_query: Query<(Entity, &Physics, &GroundItem)>,
    mut inventory: ResMut<Inventory>,
    items: Res<Items>,
) {
    for (player, player_physics) in player_query.iter() {
        for (item_entity, item_physics, ground_item) in item_query.iter() {
            let storage_item = StorageItem {
                item_id: ground_item.item_id.clone(),
                stack_count: ground_item.stack_count,
            };
            if item_physics.position.distance(player_physics.position) < player.item_pick_up_radius
            {
                let target_slots = inventory
                    .storage
                    .get_target_slots(items.store, &storage_item);
                let can_pick_up = target_slots.len() > 0;
                if can_pick_up {
                    inventory.storage.add_item(items.store, &storage_item);
                    commands.entity(item_entity).despawn();
                }
                let pick_up_message = if can_pick_up {
                    format!("Picked up {}.", ground_item.item_id)
                } else {
                    format!("Can't pick up {}. Inventory full.", ground_item.item_id)
                };
                println!("Player Inventory: {}", pick_up_message);
                for i in 0..inventory.storage.capacity {
                    let slot = inventory.storage.items.get(&i).unwrap();
                    let text = match slot {
                        Some(item) => format!("{}({})", item.item_id, item.stack_count),
                        None => "Empty".into(),
                    };
                    println!("Slot {}: {}", i, text);
                }
            }
        }
    }
}
