use bevy::prelude::*;

use crate::{resources::inventory::Inventory, systems::inventory::pick_up_ground_items};

pub struct IronWildsInventoryPlugin;
impl Plugin for IronWildsInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .register_type::<Inventory>()
            .add_systems(Update, pick_up_ground_items);
    }
}
