use bevy::prelude::*;

use crate::resources::items::ItemStore;
use crate::systems::items::spawn_items;

pub struct IronWildsItemsPlugin;
impl Plugin for IronWildsItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemStore>()
            .add_systems(Startup, spawn_items);
    }
}
