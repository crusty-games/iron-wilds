use bevy::prelude::*;

use crate::components::items::GroundItem;
use crate::components::player::{Player, PrimaryPlayer};
use crate::components::{physics::Physics, storage::StorageItem};
use crate::resources::inventory::Inventory;
use crate::resources::items::ItemStore;

pub fn pick_up_ground_items(
    mut commands: Commands,
    player_query: Query<(&Player, &Physics), With<PrimaryPlayer>>,
    mut item_query: Query<(Entity, &Physics, &mut GroundItem)>,
    mut inventory: ResMut<Inventory>,
    item_store: Res<ItemStore>,
) {
    for (player, player_physics) in player_query.iter() {
        for (item_entity, item_physics, mut ground_item) in item_query.iter_mut() {
            if !ground_item.pick_up_timeout.finished() {
                continue;
            }
            let storage_item = StorageItem {
                item_id: ground_item.item_id.clone(),
                stack_count: ground_item.stack_count,
            };
            if item_physics.position.distance(player_physics.position) < player.item_pick_up_radius
            {
                let transaction = inventory
                    .storage
                    .get_target_slots(&item_store, &storage_item);
                let can_pick_up = transaction.target_slots.len() > 0;
                if can_pick_up {
                    inventory.storage.commit_add(&item_store, &transaction);
                    if transaction.stack_left == 0 {
                        commands.entity(item_entity).despawn();
                    } else {
                        ground_item.stack_count = transaction.stack_left;
                    }
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
