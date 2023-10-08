use bevy::prelude::*;

use crate::resources::items::ItemStore;

pub fn spawn_items(mut commands: Commands, item_store: Res<ItemStore>) {
    for item in item_store.items.iter() {
        item.spawn(&mut commands);
    }
}
