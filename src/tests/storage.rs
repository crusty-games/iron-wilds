#[cfg(test)]
mod storage {
    use std::collections::HashMap;

    use crate::components::storage::{Storage, StorageItem};
    use crate::game::items::{store::ItemStore, Item};

    const BREAD: &str = "bread";
    const SWORD: &str = "sword";
    const STONE: &str = "stone";
    const CAPACITY: usize = 4;

    fn make_store() -> ItemStore {
        let mut item_store = ItemStore {
            items: HashMap::new(),
        };

        let id = String::from(BREAD);
        item_store.items.insert(
            id.clone(),
            Item {
                id,
                max_stack_count: 4,
                ..Default::default()
            },
        );

        let id = String::from(SWORD);
        item_store.items.insert(
            id.clone(),
            Item {
                id,
                max_stack_count: 1,
                ..Default::default()
            },
        );

        let id = String::from(STONE);
        item_store.items.insert(
            id.clone(),
            Item {
                id,
                max_stack_count: 12,
                ..Default::default()
            },
        );

        item_store
    }

    #[test]
    fn test_add_and_fit_items() {
        let mut test_storage = Storage::new(CAPACITY);
        let item_store = make_store();

        assert_eq!(test_storage.items.len(), CAPACITY);
        for slot_index in 0..CAPACITY {
            assert!(matches!(test_storage.items.get(&slot_index).unwrap(), None));
        }

        test_storage.add_item(
            &item_store,
            &StorageItem {
                item_id: STONE.into(),
                stack_count: 8,
            },
        );
        let first_slot = test_storage.items.get(&0).unwrap().clone().unwrap();
        assert_eq!(first_slot.item_id, STONE);
        assert_eq!(first_slot.stack_count, 8);
        for slot_index in 1..CAPACITY {
            assert!(matches!(test_storage.items.get(&slot_index).unwrap(), None));
        }

        test_storage.add_item(
            &item_store,
            &StorageItem {
                item_id: SWORD.into(),
                stack_count: 2,
            },
        );
        let first_slot = test_storage.items.get(&0).unwrap().clone().unwrap();
        assert_eq!(first_slot.item_id, STONE);
        assert_eq!(first_slot.stack_count, 8);

        let second_slot = test_storage.items.get(&1).unwrap().clone().unwrap();
        assert_eq!(second_slot.item_id, SWORD);
        assert_eq!(second_slot.stack_count, 1);

        let third_slot = test_storage.items.get(&2).unwrap().clone().unwrap();
        assert_eq!(third_slot.item_id, SWORD);
        assert_eq!(third_slot.stack_count, 1);

        assert!(matches!(test_storage.items.get(&3).unwrap(), None));

        test_storage.add_item(
            &item_store,
            &StorageItem {
                item_id: STONE.into(),
                stack_count: 6,
            },
        );
        let first_slot = test_storage.items.get(&0).unwrap().clone().unwrap();
        assert_eq!(first_slot.item_id, STONE);
        assert_eq!(first_slot.stack_count, 12);

        let second_slot = test_storage.items.get(&1).unwrap().clone().unwrap();
        assert_eq!(second_slot.item_id, SWORD);
        assert_eq!(second_slot.stack_count, 1);

        let third_slot = test_storage.items.get(&2).unwrap().clone().unwrap();
        assert_eq!(third_slot.item_id, SWORD);
        assert_eq!(third_slot.stack_count, 1);

        let fourth_slot = test_storage.items.get(&3).unwrap().clone().unwrap();
        assert_eq!(fourth_slot.item_id, STONE);
        assert_eq!(fourth_slot.stack_count, 2);

        let try_full = test_storage.get_target_slots(
            &item_store,
            &StorageItem {
                item_id: BREAD.into(),
                stack_count: 4,
            },
        );
        assert_eq!(try_full.len(), 0);
    }
}
