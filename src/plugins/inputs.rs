use bevy::prelude::*;

use crate::systems::inputs::choose_active_slot_keyboard;
use crate::systems::inputs::choose_active_slot_scroll;
use crate::systems::inputs::click_inventory_slot;
use crate::systems::inputs::drop_item;
use crate::systems::inputs::move_player;

pub struct IronWildsInputsPlugin;
impl Plugin for IronWildsInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_player,
                drop_item,
                choose_active_slot_keyboard,
                choose_active_slot_scroll,
                click_inventory_slot,
            ),
        );
    }
}
