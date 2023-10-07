pub mod game_items;
pub mod plugin;
mod tests;

#[derive(Clone, Copy)]
pub enum GameItemComponent {
    Placable,
    Pickupable,
    Stackable { max_stack: usize },
    Equipable,
}

#[derive(Clone)]
pub struct GameItem {
    pub id: String,
    pub name: String,
    pub components: Vec<GameItemComponent>,
}

impl Default for GameItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            components: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct InventoryItem {
    pub game_item: GameItem,
    pub count: usize,
}

#[macro_export]
macro_rules! item_has_component {
    ( $item:expr, $pattern:pat, $then:block ) => {
        item_has_component!($item, $pattern, $then, {})
    };
    ( $item:expr, $pattern:pat, $then:block, $else:block ) => {
        #[allow(unused_variables)]
        let opt = $item.components.iter().find(|comp| match comp {
            $pattern => true,
            _ => false,
        });
        if let Some($pattern) = opt $then else $else
    };
}
