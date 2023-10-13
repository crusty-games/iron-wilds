use bevy::prelude::*;

use crate::components::items::GroundItem;
use crate::components::player::{Player, PrimaryPlayer};
use crate::components::{physics::Physics, storage::StorageItem};
use crate::events::inventory::LogInventoryEvent;
use crate::resources::inventory::{Inventory, INVENTORY_COLUMNS, INVENTORY_ROWS};
use crate::resources::items::ItemStore;

pub fn pick_up_ground_items(
    mut commands: Commands,
    player_query: Query<(&Player, &Physics), With<PrimaryPlayer>>,
    mut item_query: Query<(Entity, &Physics, &mut GroundItem)>,
    mut inventory: ResMut<Inventory>,
    item_store: Res<ItemStore>,
    mut log_inventory_event: EventWriter<LogInventoryEvent>,
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
                if transaction.target_slots.len() > 0 {
                    inventory.storage.commit_add(&item_store, &transaction);
                    if transaction.stack_left == 0 {
                        commands.entity(item_entity).despawn();
                    } else {
                        ground_item.stack_count = transaction.stack_left;
                    }
                    log_inventory_event.send(LogInventoryEvent);
                }
            }
        }
    }
}

pub fn log_inventory(
    inventory: Res<Inventory>,
    log_inventory_event: EventReader<LogInventoryEvent>,
) {
    if log_inventory_event.len() == 0 {
        return;
    }
    let pad = "        ";
    let divider = ".";
    let mut log = String::from("Inventory:\n");

    for row in 0..INVENTORY_ROWS {
        let mut row_text = String::new();
        for column in 0..INVENTORY_COLUMNS {
            let index = row * INVENTORY_ROWS + column;
            let storage_item = inventory.storage.items.get(&index).unwrap();
            let item_text = if let Some(StorageItem {
                item_id,
                stack_count,
            }) = storage_item
            {
                format!("{}({})", item_id, stack_count)
            } else {
                pad.clone().to_string()
            };
            let item_text = &format!("{}{}", item_text, pad)[0..pad.len()];
            let item_text = if row == 0 && column == inventory.hotbar.active_slot {
                format!("[{}]", item_text)
            } else {
                format!(" {} ", item_text)
            };
            row_text = format!("{}{}{}", row_text, divider, item_text);
        }
        log = format!("{}{}{}\n", log, row_text, divider);
    }

    let _ = clearscreen::clear();
    println!("{}", log);
}
