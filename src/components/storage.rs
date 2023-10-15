use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use std::collections::HashMap;
use std::ops::{Add, Range};

use crate::resources::items::ItemStore;

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Storage {
    pub capacity: usize,
    pub items: HashMap<usize, Option<StorageItem>>,
}

#[derive(Clone, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct StorageItem {
    pub item_id: String,
    pub stack_count: usize,
}

pub struct TargetSlot {
    pub stack_count: usize,
    pub slot_index: usize,
}

pub struct AddItem {
    pub item_id: String,
    pub target_slots: Vec<TargetSlot>,
    pub stack_left: usize,
}

impl Storage {
    pub fn new(capacity: usize) -> Self {
        let mut items = HashMap::new();
        for slot_index in 0..capacity {
            items.insert(slot_index, None);
        }
        Self { capacity, items }
    }

    pub fn range(&self) -> Range<usize> {
        0..self.capacity
    }

    pub fn add_item(&mut self, item_store: &ItemStore, storage_item: &StorageItem) -> AddItem {
        let transaction = self.get_target_slots(item_store, storage_item);
        self.commit_add(item_store, &transaction);
        transaction
    }

    pub fn commit_add(&mut self, item_store: &ItemStore, transaction: &AddItem) {
        let AddItem {
            item_id,
            target_slots,
            ..
        } = transaction;
        let item = item_store.get(item_id);
        for fit in target_slots.iter() {
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

    pub fn get_target_slots(&self, item_store: &ItemStore, storage_item: &StorageItem) -> AddItem {
        let StorageItem {
            item_id,
            stack_count,
        } = storage_item;

        let item = item_store.get(item_id);
        let mut target_slots: Vec<TargetSlot> = Vec::new();
        let mut stack_left = stack_count.to_owned();
        for slot_index in self.range() {
            let storage_item = self.items.get(&slot_index).unwrap();
            if stack_left == 0 {
                continue;
            }
            if let Some(StorageItem {
                item_id: storage_item_id,
                stack_count: storage_stack_count,
            }) = storage_item
            {
                if &item.id == storage_item_id {
                    let stack_taken = stack_left.min(item.max_stack_count - storage_stack_count);
                    if stack_taken > 0 {
                        stack_left -= stack_taken;
                        target_slots.push(TargetSlot {
                            stack_count: stack_taken,
                            slot_index,
                        })
                    }
                }
            } else {
                let stack_taken = stack_left.min(item.max_stack_count);
                if stack_taken > 0 {
                    stack_left -= stack_taken;
                    target_slots.push(TargetSlot {
                        stack_count: stack_taken,
                        slot_index,
                    })
                }
            }
        }
        AddItem {
            item_id: item_id.clone(),
            target_slots,
            stack_left,
        }
    }
}
