use bevy::prelude::*;

use crate::systems::ui::{inventory_ui, spawn_inventory_ui};

pub struct IronWildsUiPlugin;
impl Plugin for IronWildsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inventory_ui)
            .add_systems(Update, inventory_ui);
    }
}
