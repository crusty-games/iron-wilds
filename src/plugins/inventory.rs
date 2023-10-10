use bevy::prelude::*;

use crate::resources::inventory::PrimaryPlayerInventory;

pub struct IronWildsInventoryPlugin;
impl Plugin for IronWildsInventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrimaryPlayerInventory>();
    }
}
