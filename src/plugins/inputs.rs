use bevy::prelude::*;

use crate::resources::inputs::GameInputs;
use crate::systems::inputs::choose_active_slot_controller;
use crate::systems::inputs::choose_active_slot_keyboard;
use crate::systems::inputs::choose_active_slot_scroll;
use crate::systems::inputs::click_inventory_slot;
use crate::systems::inputs::drop_item;
use crate::systems::inputs::movement_controller;
use crate::systems::inputs::movement_keyboard;
use crate::systems::player::PlayerSet;

pub struct IronWildsInputsPlugin;
impl Plugin for IronWildsInputsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameInputs>().add_systems(
            Update,
            (
                movement_keyboard.before(PlayerSet::Movement),
                movement_controller.before(PlayerSet::Movement),
                drop_item,
                choose_active_slot_keyboard,
                choose_active_slot_scroll,
                choose_active_slot_controller,
                click_inventory_slot,
            ),
        );
    }
}
