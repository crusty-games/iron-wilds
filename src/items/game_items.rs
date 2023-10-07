use super::GameItem;

pub struct GameItemLoader {
    items: Vec<GameItem>,
}

impl GameItemLoader {
    pub fn new() -> Self {
        Self {
            items: Self::load(),
        }
    }

    pub fn get_by_id(&self, id: String) -> &GameItem {
        let item_query = self.items.iter().find(|i| i.id == id);
        if let Some(item) = item_query {
            item
        } else {
            panic!("get_by_id(\"{}\") could not be found", id)
        }
    }

    fn load() -> Vec<GameItem> {
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
pub fn test_get_item() {
    let item_loader = GameItemLoader::new();
    item_loader.get_by_id("iron".into());
}

#[test]
#[should_panic]
pub fn test_get_unknown_item() {
    let item_loader = GameItemLoader::new();
    item_loader.get_by_id("item_not_in_game".into());
}
