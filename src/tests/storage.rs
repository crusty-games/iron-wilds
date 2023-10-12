use crate::{
    components::storage::{Storage, StorageItem},
    tests::items::{make_test_store, ID_BREAD, ID_STONE, ID_SWORD},
};

const STORE_CAPACITY: usize = 4;

#[test]
fn test_add_and_fit_items() {
    let mut test_storage = Storage::new(STORE_CAPACITY);
    let item_store = make_test_store();

    assert_eq!(test_storage.items.len(), STORE_CAPACITY);
    for slot_index in 0..STORE_CAPACITY {
        assert!(matches!(test_storage.items.get(&slot_index).unwrap(), None));
    }

    test_storage.add_item(
        &item_store,
        &StorageItem {
            item_id: ID_STONE.into(),
            stack_count: 8,
        },
    );
    let first_slot = test_storage.items.get(&0).unwrap().clone().unwrap();
    assert_eq!(first_slot.item_id, ID_STONE);
    assert_eq!(first_slot.stack_count, 8);
    for slot_index in 1..STORE_CAPACITY {
        assert!(matches!(test_storage.items.get(&slot_index).unwrap(), None));
    }

    test_storage.add_item(
        &item_store,
        &StorageItem {
            item_id: ID_SWORD.into(),
            stack_count: 2,
        },
    );
    let first_slot = test_storage.items.get(&0).unwrap().clone().unwrap();
    assert_eq!(first_slot.item_id, ID_STONE);
    assert_eq!(first_slot.stack_count, 8);

    let second_slot = test_storage.items.get(&1).unwrap().clone().unwrap();
    assert_eq!(second_slot.item_id, ID_SWORD);
    assert_eq!(second_slot.stack_count, 1);

    let third_slot = test_storage.items.get(&2).unwrap().clone().unwrap();
    assert_eq!(third_slot.item_id, ID_SWORD);
    assert_eq!(third_slot.stack_count, 1);

    assert!(matches!(test_storage.items.get(&3).unwrap(), None));

    test_storage.add_item(
        &item_store,
        &StorageItem {
            item_id: ID_STONE.into(),
            stack_count: 6,
        },
    );
    let first_slot = test_storage.items.get(&0).unwrap().clone().unwrap();
    assert_eq!(first_slot.item_id, ID_STONE);
    assert_eq!(first_slot.stack_count, 12);

    let second_slot = test_storage.items.get(&1).unwrap().clone().unwrap();
    assert_eq!(second_slot.item_id, ID_SWORD);
    assert_eq!(second_slot.stack_count, 1);

    let third_slot = test_storage.items.get(&2).unwrap().clone().unwrap();
    assert_eq!(third_slot.item_id, ID_SWORD);
    assert_eq!(third_slot.stack_count, 1);

    let fourth_slot = test_storage.items.get(&3).unwrap().clone().unwrap();
    assert_eq!(fourth_slot.item_id, ID_STONE);
    assert_eq!(fourth_slot.stack_count, 2);

    let transaction = test_storage.get_target_slots(
        &item_store,
        &StorageItem {
            item_id: ID_BREAD.into(),
            stack_count: 4,
        },
    );
    assert_eq!(transaction.target_slots.len(), 0);
    assert_eq!(transaction.stack_left, 4);
}
