#[cfg(test)]
mod item_tests {
    use crate::{
        item_has_component,
        items::{game_items::GameItemLoader, GameItem, GameItemComponent},
    };

    #[test]
    fn test_macro() {
        let test_value = 36;
        let item: GameItem = GameItem {
            id: "test".into(),
            name: "Test".into(),
            components: vec![
                GameItemComponent::Stackable {
                    max_stack: test_value,
                },
                GameItemComponent::Placable,
            ],
        };

        item_has_component!(item, GameItemComponent::Pickupable, { unreachable!() });
        item_has_component!(
            item,
            GameItemComponent::Stackable { max_stack },
            {
                assert_eq!(max_stack, &test_value);
            },
            { unreachable!() }
        );
    }

    #[test]
    fn test_no_item_duplicates() {
        let item_loader = GameItemLoader::new();
        let items = item_loader.items;
        let mut ids: Vec<&String> = vec![];
        let mut names: Vec<&String> = vec![];

        for item in items.iter() {
            if let Some(_) = ids.iter().find(|s| s == &&&item.id) {
                panic!("Item ID repeated: {}", item.id)
            }
            ids.push(&item.id);

            if let Some(_) = names.iter().find(|s| s == &&&item.name) {
                panic!("Item name repeated: {}", item.name)
            }
            names.push(&item.name);
        }
    }

    #[test]
    fn test_get_item() {
        let item_loader = GameItemLoader::new();
        item_loader.get_by_id("iron".into());
    }

    #[test]
    #[should_panic]
    fn test_get_unknown_item() {
        let item_loader = GameItemLoader::new();
        item_loader.get_by_id("item_not_in_game".into());
    }
}
