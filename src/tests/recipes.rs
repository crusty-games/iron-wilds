use crate::{
    components::storage::{Storage, StorageItem},
    resources::recipes::Recipe,
};

use super::items::{make_test_store, ID_BREAD, ID_STONE, ID_SWORD};

#[test]
fn test_can_craft_recipes() {
    let item_store = make_test_store();
    let recipe = Recipe::new(vec![(ID_STONE, 2), (ID_SWORD, 1)], (ID_BREAD, 1));
    let mut storage = Storage::new(4);
    storage.add_item(
        &item_store,
        &StorageItem {
            item_id: ID_STONE.to_string(),
            stack_count: 2,
        },
    );
    storage.add_item(
        &item_store,
        &StorageItem {
            item_id: ID_SWORD.to_string(),
            stack_count: 1,
        },
    );
    let can_craft = recipe.can_make_in_storage(&storage);
    assert_eq!(can_craft, true);
}
