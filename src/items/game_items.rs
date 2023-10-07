use lazy_static::lazy_static;

use super::GameItem;

lazy_static! {
    pub static ref GAME_ITEM_LOADER: GameItemLoader = GameItemLoader::new();
}
pub struct GameItemLoader {
    pub items: Vec<GameItem>,
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
            panic!("GameItem with ID {} not found", id)
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
