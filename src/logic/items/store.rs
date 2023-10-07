use lazy_static::lazy_static;

use super::{GameItem, PropFood, PropMaterial};

lazy_static! {
    pub static ref GAME_ITEM_STORE: GameItemStore = GameItemStore::new();
}
pub struct GameItemStore {
    pub items: Vec<GameItem>,
}

impl GameItemStore {
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
            panic!("GameItem with ID {} not found", id)
        }
    }

    fn load() -> Vec<GameItem> {
        let mut items: Vec<GameItem> = vec![];

        items.push(GameItem {
            id: "bread".into(),
            name: "Bread".into(),
            prop_food: Some(PropFood {
                health_increase: 20.0,
            }),
            prop_material: Some(PropMaterial),
        });

        Self::check_for_dupes(&items);

        items
    }

    fn check_for_dupes(items: &Vec<GameItem>) {
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
}
