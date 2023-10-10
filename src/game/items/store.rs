use std::collections::HashMap;

use super::{load::load_sample_items, Item};

lazy_static! {
    pub static ref ITEM_STORE: ItemStore = ItemStore::new();
}

pub struct ItemStore {
    pub items: HashMap<String, Item>,
}

impl ItemStore {
    pub fn new() -> Self {
        Self {
            items: Self::load_items(),
        }
    }

    fn load_items() -> HashMap<String, Item> {
        let mut items: HashMap<String, Item> = HashMap::new();
        for item in load_sample_items().into_iter() {
            items.insert(item.id.clone(), item);
        }
        items
    }

    pub fn get<S: AsRef<str>>(&self, id: S) -> &Item {
        let id_ref = id.as_ref();
        match self.items.get(id_ref) {
            Some(item) => item,
            None => panic!("Item by ID \"{id_ref}\" not found"),
        }
    }
}

pub struct WithItemStore<'a, T> {
    pub item_store: &'a ItemStore,
    pub payload: T,
}

impl<'a, T> WithItemStore<'a, T> {
    pub fn global(payload: T) -> WithItemStore<'a, T> {
        Self {
            item_store: &ITEM_STORE,
            payload,
        }
    }

    #[cfg(test)]
    pub fn injected(item_store: &'a ItemStore, payload: T) -> Self {
        Self {
            item_store,
            payload,
        }
    }
}
