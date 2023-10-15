use std::ops::Add;

use bevy::input::gamepad::GamepadEvent;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::components::physics::Physics;
use crate::components::player::{Player, PrimaryPlayer};
use crate::components::storage::StorageItem;
use crate::components::ui::InventorySlot;
use crate::events::items::{SpawnItemEvent, SpawnKind};
use crate::resources::inputs::GameInputs;
use crate::resources::inventory::Inventory;

pub fn movement_keyboard(keyboard_input: Res<Input<KeyCode>>, mut game_inputs: ResMut<GameInputs>) {
    if !keyboard_input.is_changed() {
        return;
    }
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
    game_inputs.movement.keyboard = if total_vec.length() == 0.0 {
        total_vec
    } else {
        total_vec.normalize()
    };
}

pub fn movement_controller(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut game_inputs: ResMut<GameInputs>,
) {
    for event in gamepad_event.iter() {
        if let GamepadEvent::Axis(axis_event) = event {
            match axis_event.axis_type {
                GamepadAxisType::LeftStickX => game_inputs.movement.controller.x = axis_event.value,
                GamepadAxisType::LeftStickY => game_inputs.movement.controller.y = axis_event.value,
                _ => {}
            }
        }
    }
}

pub fn drop_item(
    player_query: Query<&Physics, (With<PrimaryPlayer>, With<Player>)>,
    mut spawn_event: EventWriter<SpawnItemEvent>,
    mut inventory: ResMut<Inventory>,
    keyboard_input: Res<Input<KeyCode>>,
    mut gamepad_event: EventReader<GamepadEvent>,
) {
    let mut drop_item = keyboard_input.just_pressed(KeyCode::V);

    if !drop_item {
        for event in gamepad_event.iter() {
            if let GamepadEvent::Button(button_event) = event {
                if button_event.value == 1.0
                    && matches!(button_event.button_type, GamepadButtonType::West)
                {
                    drop_item = true;
                }
            }
        }
    }

    if !drop_item {
        return;
    }

    let index = inventory.hotbar.active_slot;
    for Physics { position, .. } in player_query.iter() {
        if let Some(StorageItem {
            item_id,
            stack_count,
        }) = inventory.storage.items.get(&index).unwrap()
        {
            spawn_event.send(SpawnItemEvent {
                kind: SpawnKind::GroundLoot {
                    item_id: item_id.clone(),
                    stack_count: *stack_count,
                    position: *position,
                },
            });
            *inventory.storage.items.get_mut(&index).unwrap() = None;
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

pub fn choose_active_slot_controller(
    mut inventory: ResMut<Inventory>,
    mut gamepad_event: EventReader<GamepadEvent>,
) {
    for event in gamepad_event.iter() {
        let mut change = 0;
        if let GamepadEvent::Button(button_event) = event {
            if button_event.value == 1.0 {
                match button_event.button_type {
                    GamepadButtonType::LeftTrigger => change -= 1,
                    GamepadButtonType::RightTrigger => change += 1,
                    _ => {}
                }
            }
        };
        if change != 0 {
            let range = inventory.hotbar.range();
            inventory.hotbar.active_slot = (inventory.hotbar.active_slot as isize)
                .add(change)
                .clamp(range.start as isize, range.end as isize)
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
