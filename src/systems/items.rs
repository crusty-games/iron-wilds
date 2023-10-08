use bevy::prelude::*;

use crate::resources::items::ItemStore;

pub fn spawn_items(mut commands: Commands, item_store: Res<ItemStore>) {
    for item in item_store.items.iter() {
        item.spawn_as_ground_item(&mut commands, Vec2::ZERO);
    }

    item_store
        .get_by_id("bread".into())
        .spawn_as_ground_item(&mut commands, Vec2::ZERO);
}
