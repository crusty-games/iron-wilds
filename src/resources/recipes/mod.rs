use bevy::prelude::*;

use crate::components::storage::{Storage, StorageItem};
use crate::data::load_recipe_crafting::load_sample_recipes;

#[derive(Resource)]
pub struct Recipes {
    pub crafting: Vec<Recipe>,
    pub cooking: Vec<Recipe>,
}

impl Default for Recipes {
    fn default() -> Self {
        Self {
            crafting: load_sample_recipes(),
            cooking: load_sample_recipes(),
        }
    }
}

pub struct Recipe {
    pub ingredients: Vec<StorageItem>,
    pub output: StorageItem,
}

impl Recipe {
    pub fn new(i: Vec<(&str, usize)>, output: (&str, usize)) -> Self {
        let mut ingredients: Vec<StorageItem> = Vec::new();
        for ing in i.iter() {
            ingredients.push(StorageItem {
                item_id: ing.0.into(),
                stack_count: ing.1,
            })
        }
        Self {
            ingredients,
            output: StorageItem {
                item_id: output.0.into(),
                stack_count: output.1,
            },
        }
    }

    pub fn can_make_in_storage(&self, storage: &Storage) -> bool {
        self.ingredients.iter().all(|ing| {
            storage
                .items
                .values()
                .find(|slot| match slot {
                    Some(item) => {
                        item.item_id == ing.item_id && item.stack_count <= ing.stack_count
                    }
                    None => false,
                })
                .is_some()
        })
    }
}
