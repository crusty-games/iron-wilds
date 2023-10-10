use std::ops::Add;

use bevy::{prelude::*, utils::HashMap};

use crate::game::items::store::{ItemStore, ITEM_STORE};

#[derive(Clone, Debug)]
pub struct StorageItem {
    pub item_id: String,
    pub stack_count: usize,
}

#[derive(Debug)]
pub struct StorageItemCanFit {
    pub stack_count: usize,
    pub slot_index: usize,
}

#[derive(Component)]
pub struct Storage<'a> {
    pub capacity: usize,
    pub items: HashMap<usize, Option<StorageItem>>,
    item_store: &'a ItemStore,
}

impl<'a> Storage<'a> {
    pub fn new(capacity: usize) -> Self {
        let mut items = HashMap::new();
        for slot_index in 0..capacity {
            items.insert(slot_index, None);
        }
        Self {
            capacity,
            items,
            item_store: &ITEM_STORE,
        }
    }

    #[cfg(test)]
    pub fn set_item_store(&mut self, item_store: &'a ItemStore) {
        self.item_store = item_store;
    }

    pub fn add_item<S: AsRef<str>>(&mut self, item_id: S, stack_count: usize) {
        let item = self.item_store.get(&item_id);
        let can_fit = self.can_fit(item_id, stack_count);
        for fit in can_fit.iter() {
            match self.items.get_mut(&fit.slot_index).unwrap() {
                Some(StorageItem {
                    item_id: storage_item_id,
                    stack_count: storage_stack_count,
                }) => {
                    if storage_item_id != &item.id {
                        panic!(
                            "Storage tried to merge item {} with {}",
                            item.id, storage_item_id
                        );
                    }
                    *storage_stack_count = storage_stack_count.add(fit.stack_count);
                    if *storage_stack_count > item.max_stack_count {
                        panic!(
                            "Storage merge of {} exceeded max stack count",
                            storage_item_id
                        )
                    }
                }
                _ => {
                    if fit.stack_count > item.max_stack_count {
                        panic!("Storage add of {} exceeded max stack count", item.id)
                    }
                    self.items.insert(
                        fit.slot_index,
                        Some(StorageItem {
                            item_id: item.id.clone(),
                            stack_count: fit.stack_count,
                        }),
                    );
                }
            }
        }
    }

    pub fn can_fit<S: AsRef<str>>(&self, item_id: S, stack_count: usize) -> Vec<StorageItemCanFit> {
        let item = self.item_store.get(&item_id);
        let mut can_fit: Vec<StorageItemCanFit> = Vec::new();
        let mut stack_left = stack_count.clone();
        for slot_index in 0..self.capacity {
            let storage_item = self.items.get(&slot_index).unwrap();
            if stack_left <= 0 {
                continue;
            }
            if let Some(StorageItem {
                item_id: storage_item_id,
                stack_count: storage_stack_count,
            }) = storage_item
            {
                if &item.id == storage_item_id {
                    let stack_taken = stack_left.min(&item.max_stack_count - storage_stack_count);
                    if stack_taken > 0 {
                        stack_left -= stack_taken;
                        can_fit.push(StorageItemCanFit {
                            stack_count: stack_taken,
                            slot_index: slot_index.clone(),
                        })
                    }
                }
            } else {
                let stack_taken = stack_left.min(item.max_stack_count);
                if stack_taken > 0 {
                    stack_left -= stack_taken;
                    can_fit.push(StorageItemCanFit {
                        stack_count: stack_taken,
                        slot_index: slot_index.clone(),
                    })
                }
            }
        }
        can_fit
    }
}
