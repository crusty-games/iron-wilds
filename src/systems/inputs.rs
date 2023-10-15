use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::components::physics::Physics;
use crate::components::player::{Player, PrimaryPlayer};
use crate::components::storage::StorageItem;
use crate::components::ui::InventorySlot;
use crate::events::items::{SpawnItemEvent, SpawnKind};
use crate::resources::inventory::Inventory;
use crate::resources::physics::PhysicsTimer;

pub fn move_player(
    mut player_query: Query<(&Player, &mut Physics)>,
    keyboard_input: Res<Input<KeyCode>>,
    physics_timer: Res<PhysicsTimer>,
) {
    if physics_timer.main_tick.finished() {
        for (player, mut object) in player_query.iter_mut() {
            let mut total_vec = Vec2::ZERO;
            if keyboard_input.pressed(KeyCode::W) {
                total_vec.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::S) {
                total_vec.y -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::D) {
                total_vec.x += 1.0;
            }
            if keyboard_input.pressed(KeyCode::A) {
                total_vec.x -= 1.0;
            }
            if total_vec.length() > 0.0 {
                total_vec = total_vec.normalize() * player.movement_speed;
                object.velocity += total_vec;
            }
        }
    }
}

pub fn drop_item(
    player_query: Query<&Physics, (With<PrimaryPlayer>, With<Player>)>,
    mut spawn_event: EventWriter<SpawnItemEvent>,
    mut inventory: ResMut<Inventory>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let index = inventory.hotbar.active_slot;
    for Physics { position, .. } in player_query.iter() {
        if keyboard_input.just_pressed(KeyCode::V) {
            if let Some(StorageItem {
                item_id,
                stack_count,
            }) = inventory.storage.items.get(&index).unwrap()
            {
                spawn_event.send(SpawnItemEvent {
                    kind: SpawnKind::GroundLoot {
                        item_id: item_id.clone(),
                        stack_count: stack_count.to_owned(),
                        position: position.to_owned(),
                    },
                });
                *inventory.storage.items.get_mut(&index).unwrap() = None;
            }
        }
    }
}

macro_rules! choose_key_slot {
    ($keyboard_input:ident, $inventory:ident, $key:ident, $index:expr) => {
        if $keyboard_input.just_pressed(KeyCode::$key) {
            $inventory.hotbar.active_slot = $index;
        }
    };
}

pub fn choose_active_slot_keyboard(
    mut inventory: ResMut<Inventory>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    choose_key_slot!(keyboard_input, inventory, Key1, 0);
    choose_key_slot!(keyboard_input, inventory, Key2, 1);
    choose_key_slot!(keyboard_input, inventory, Key3, 2);
    choose_key_slot!(keyboard_input, inventory, Key4, 3);
    choose_key_slot!(keyboard_input, inventory, Key5, 4);
    choose_key_slot!(keyboard_input, inventory, Key6, 5);
    choose_key_slot!(keyboard_input, inventory, Key7, 6);
    choose_key_slot!(keyboard_input, inventory, Key8, 7);
}

pub fn choose_active_slot_scroll(
    mut inventory: ResMut<Inventory>,
    mut scroll_event: EventReader<MouseWheel>,
) {
    for ev in scroll_event.iter() {
        if let MouseScrollUnit::Line = ev.unit {
            inventory.hotbar.active_slot = ((inventory.hotbar.active_slot as i32) - (ev.y as i32))
                .max(0)
                .min((inventory.hotbar.capacity as i32) - 1)
                as usize;
        }
    }
}

pub fn click_inventory_slot(
    slots_query: Query<(&InventorySlot, &Interaction)>,
    mut inventory: ResMut<Inventory>,
) {
    for (slot, interaction) in slots_query.iter() {
        if matches!(interaction, Interaction::Pressed) {
            inventory.hotbar.active_slot = slot.slot_index;
        }
    }
}
