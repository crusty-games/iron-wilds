use bevy::prelude::*;

use crate::{
    resources::inventory::PrimaryPlayerInventory, systems::inventory::pick_up_ground_items,
};

pub struct IronWildsInventoryPlugin;
impl Plugin for IronWildsInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrimaryPlayerInventory>()
            .add_systems(Update, pick_up_ground_items);
    }
}
