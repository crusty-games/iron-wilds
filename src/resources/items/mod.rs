pub mod config;
pub mod load;

use std::collections::HashMap;

use bevy::prelude::*;

use self::{config::ItemConfig, load::load_sample_items};

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

impl ItemStore {
    fn load_items() -> HashMap<String, ItemConfig> {
        let mut items: HashMap<String, ItemConfig> = HashMap::new();
        for item in load_sample_items().into_iter() {
            items.insert(item.id.clone(), item);
        }
        items
    }

    pub fn get<S: AsRef<str>>(&self, id: S) -> &ItemConfig {
        let id_ref = id.as_ref();
        match self.items.get(id_ref) {
            Some(item) => item,
            None => panic!("Item by ID \"{id_ref}\" not found"),
        }
    }
}
