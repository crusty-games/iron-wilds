use std::collections::HashMap;

use crate::resources::items::{config::ItemConfig, ItemStore};

pub const ID_BREAD: &str = "bread";
pub const ID_SWORD: &str = "sword";
pub const ID_STONE: &str = "stone";

pub fn make_test_store() -> ItemStore {
    let mut item_store = ItemStore {
        items: HashMap::new(),
    };

    let id = String::from(ID_BREAD);
    item_store.items.insert(
        id.clone(),
        ItemConfig {
            id,
            max_stack_count: 4,
            ..Default::default()
        },
    );

    let id = String::from(ID_SWORD);
    item_store.items.insert(
        id.clone(),
        ItemConfig {
            id,
            max_stack_count: 1,
            ..Default::default()
        },
    );

    let id = String::from(ID_STONE);
    item_store.items.insert(
        id.clone(),
        ItemConfig {
            id,
            max_stack_count: 12,
            ..Default::default()
        },
    );

    item_store
}

#[test]
fn test_get() {
    let item_store = make_test_store();
    let item = item_store.get(ID_STONE);
    assert_eq!(item.id, ID_STONE);
}

#[test]
#[should_panic]
fn test_get_unknown() {
    let item_store = make_test_store();
    item_store.get("doesnt_exist");
}
