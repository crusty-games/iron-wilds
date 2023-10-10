use bevy::prelude::*;

use crate::{
    components::{
        items::GroundItem,
        physics::Physics,
        player::{Player, PrimaryPlayer},
    },
    resources::inventory::PrimaryPlayerInventory,
};

pub fn pick_up_ground_items(
    mut commands: Commands,
    primary_player_query: Query<(&Player, &Physics), With<PrimaryPlayer>>,
    item_query: Query<(Entity, &Physics, &GroundItem)>,
    mut player_inventory: ResMut<PrimaryPlayerInventory<'static>>,
) {
    for (player, primary_player_physics) in primary_player_query.iter() {
        for (item_entity, item_physics, ground_item) in item_query.iter() {
            if item_physics
                .position
                .distance(primary_player_physics.position)
                < player.item_pick_up_radius
            {
                let can_fit = player_inventory
                    .storage
                    .can_fit(&ground_item.item_id, ground_item.stack_count);
                let pick_up_item = can_fit.len() > 0;
                if pick_up_item {
                    player_inventory
                        .storage
                        .add_item(&ground_item.item_id, ground_item.stack_count);
                    commands.entity(item_entity).despawn();
                }
                let message = if pick_up_item {
                    format!("Picked up {}.", ground_item.item_id)
                } else {
                    format!("Can't pick up {}. Inventory full.", ground_item.item_id)
                };
                println!("Player Inventory: {}", message);
                for i in 0..player_inventory.storage.capacity {
                    let slot = player_inventory.storage.items.get(&i).unwrap();
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
