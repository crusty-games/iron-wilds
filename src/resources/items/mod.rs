pub mod config;
mod load_food;
mod load_weapons;

use bevy::prelude::*;
use std::collections::HashMap;

use self::{config::ItemConfig, load_food::load_food_items, load_weapons::load_weapon_items};

#[derive(Resource)]
pub struct ItemStore {
    pub items: HashMap<String, ItemConfig>,
}

impl Default for ItemStore {
    fn default() -> Self {
        Self {
            items: Self::load_items(),
        }
    }
}

macro_rules! load_items {
    ($items:ident, $load:ident) => {
        for item in $load().into_iter() {
            println!("Loaded item {}:{}", item.id, item.name);
            $items.insert(item.id.clone(), item);
        }
    };
}

impl ItemStore {
    fn load_items() -> HashMap<String, ItemConfig> {
        let mut items: HashMap<String, ItemConfig> = HashMap::new();
        load_items!(items, load_weapon_items);
        load_items!(items, load_food_items);
        items
    }

    pub fn get<S: AsRef<str>>(&self, id: S) -> &ItemConfig {
        let id_ref = id.as_ref();
        match self.items.get(id_ref) {
            Some(item) => item,
            None => panic!("Item by ID \"${id_ref}\" not found"),
        }
    }
}
