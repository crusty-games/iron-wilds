use bevy::prelude::*;

use crate::{components::items::GroundItem, resources::items::ItemStore};

pub fn spawn_items(mut commands: Commands, item_store: Res<ItemStore>) {
    for item in item_store.items.iter() {
        let entity = item.spawn(&mut commands);
        commands.entity(entity).insert(GroundItem::default());
    }
}
