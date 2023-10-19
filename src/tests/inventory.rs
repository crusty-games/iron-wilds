use crate::{components::storage::StorageItem, resources::inventory::Inventory};

use super::items::{make_test_store, ID_STONE};

#[test]
fn test_get_active_item() {
    let item_store = make_test_store();
    let mut inventory = Inventory::default();
    inventory.storage.add_item(
        &item_store,
        &StorageItem {
            item_id: ID_STONE.into(),
            stack_count: 1,
        },
    );

    assert_eq!(inventory.hotbar.active_slot, 0);
    assert_eq!(inventory.active_item().clone().unwrap().item_id, ID_STONE);

    inventory.hotbar.active_slot = 1;
    assert!(inventory.active_item().is_none());
}
