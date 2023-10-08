pub mod config;
mod load_food;
mod load_weapons;

use bevy::prelude::*;

use self::{config::ItemConfig, load_food::load_food_items, load_weapons::load_weapon_items};

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
        items.append(&mut load_weapon_items());
        items.append(&mut load_food_items());
        for item in items.iter() {
            println!("Item loaded: {}", item.item.id);
        }
        items
    }

    pub fn get_by_id(&self, id: String) -> &ItemConfig {
        match self.items.iter().find(|item| item.item.id == id) {
            Some(item) => item,
            None => panic!("Item by ID \"${id}\" not found"),
        }
    }
}
