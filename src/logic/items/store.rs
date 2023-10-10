use std::collections::HashMap;

use crate::logic::items::load::load_sample_items;

use super::config::Item;

lazy_static! {
    pub static ref ITEM_STORE: ItemStore = ItemStore::new();
}

pub struct ItemStore {
    pub items: HashMap<String, Item>,
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
    pub fn new() -> Self {
        Self {
            items: Self::load_items(),
        }
    }

    fn load_items() -> HashMap<String, Item> {
        let mut items: HashMap<String, Item> = HashMap::new();
        load_items!(items, load_sample_items);
        items
    }

    pub fn get<S: AsRef<str>>(&self, id: S) -> &Item {
        let id_ref = id.as_ref();
        match self.items.get(id_ref) {
            Some(item) => item,
            None => panic!("Item by ID \"${id_ref}\" not found"),
        }
    }
}
