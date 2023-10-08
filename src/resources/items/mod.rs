mod load_food;
pub mod utils;

use bevy::prelude::*;

use self::{load_food::load_food_items, utils::ItemConfig};

#[derive(Resource)]
pub struct ItemStore {
    pub items: Vec<ItemConfig>,
}

impl Default for ItemStore {
    fn default() -> Self {
        Self {
            items: Self::load_items(),
        }
    }
}

impl ItemStore {
    fn load_items() -> Vec<ItemConfig> {
        let mut items: Vec<ItemConfig> = vec![];

        items.append(&mut load_food_items());

        for item in items.iter() {
            println!("Item loaded: {}", item.item.id);
        }

        items
    }
}
