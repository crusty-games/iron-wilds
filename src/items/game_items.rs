use super::GameItem;

pub struct WorldItems;
impl WorldItems {
    pub fn load() -> Vec<GameItem> {
        let mut items: Vec<GameItem> = vec![];

        items.push(GameItem {
            id: "iron".into(),
            name: "Iron".into(),
            components: vec![],
        });

        items.push(GameItem {
            id: "gravity_gun".into(),
            name: "Gravity Gun".into(),
            components: vec![],
        });

        items
    }
}

#[test]
pub fn test_no_item_duplicates() {
    let items = WorldItems::load();
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
