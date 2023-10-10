use std::collections::HashMap;

use super::{config::ItemConfig, load::load_sample_items};

lazy_static! {
    pub static ref ITEM_STORE: ItemStore = ItemStore::new();
}

pub struct ItemStore {
    pub items: HashMap<String, ItemConfig>,
}

impl ItemStore {
    pub fn new() -> Self {
        Self {
            items: Self::load_items(),
        }
    }

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
