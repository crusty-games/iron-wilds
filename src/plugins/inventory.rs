use bevy::prelude::*;

use crate::{
    events::inventory::LogInventoryEvent,
    resources::inventory::Inventory,
    systems::{
        inputs::choose_active_slot_keyboard,
        inventory::{log_inventory, pick_up_ground_items},
    },
};

pub struct IronWildsInventoryPlugin;
impl Plugin for IronWildsInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .register_type::<Inventory>()
            .add_event::<LogInventoryEvent>()
            .add_systems(Update, pick_up_ground_items)
            .add_systems(Update, log_inventory)
            .add_systems(Update, choose_active_slot_keyboard);
    }
}
