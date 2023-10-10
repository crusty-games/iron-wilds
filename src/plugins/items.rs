use bevy::prelude::*;

use crate::events::items::SpawnItemEvent;
use crate::resources::item::Items;
use crate::systems::items::{spawn_item_event_handler, spawn_items};

pub struct IronWildsItemsPlugin;
impl Plugin for IronWildsItemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Items>()
            .add_event::<SpawnItemEvent>()
            .add_systems(Startup, spawn_items)
            .add_systems(Update, spawn_item_event_handler);
    }
}
