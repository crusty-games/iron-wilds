use bevy::prelude::*;

use crate::components::physics::Physics;
use crate::components::player::{Player, PrimaryPlayer};
use crate::components::storage::StorageItem;
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

macro_rules! drop_slot {
    ($keyboard_input:ident, $inventory:ident, $spawn_event:ident, $position:ident, $key:ident, $index:expr) => {
        if $keyboard_input.just_pressed(KeyCode::$key) {
            if let Some(StorageItem {
                item_id,
                stack_count,
            }) = $inventory.storage.items.get(&$index).unwrap()
            {
                $spawn_event.send(SpawnItemEvent {
                    kind: SpawnKind::GroundLoot {
                        item_id: item_id.clone(),
                        stack_count: stack_count.clone(),
                        position: $position.clone(),
                    },
                });
                *$inventory.storage.items.get_mut(&$index).unwrap() = None;
            }
        }
    };
}

pub fn drop_items(
    player_query: Query<(&Player, &Physics), With<PrimaryPlayer>>,
    mut spawn_event: EventWriter<SpawnItemEvent>,
    mut inventory: ResMut<Inventory>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (_, Physics { position, .. }) in player_query.iter() {
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key1, 0);
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key2, 1);
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key3, 2);
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key4, 3);
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key5, 4);
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key6, 5);
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key7, 6);
        drop_slot!(keyboard_input, inventory, spawn_event, position, Key8, 7);
    }
}
