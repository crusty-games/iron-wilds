use bevy::prelude::*;

use crate::resources::inventory::Inventory;
use crate::systems::{inputs::choose_active_slot_keyboard, inventory::pick_up_ground_items};

pub struct IronWildsInventoryPlugin;
impl Plugin for IronWildsInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .register_type::<Inventory>()
            .add_systems(Update, pick_up_ground_items)
            .add_systems(Update, choose_active_slot_keyboard);
    }
}
