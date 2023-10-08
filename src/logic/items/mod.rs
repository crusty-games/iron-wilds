pub mod store;

macro_rules! item_prop {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name;
    };
}

#[derive(Clone)]
pub struct PropFood {
    pub health_increase: f32,
}

item_prop!(PropArmor);
item_prop!(PropBlock);
item_prop!(PropTool);

#[derive(Clone)]
pub struct GameItem {
    pub id: String,
    pub name: String,

    pub prop_food: Option<PropFood>,
    pub prop_armor: Option<PropArmor>,
    pub prop_block: Option<PropBlock>,
    pub prop_tool: Option<PropTool>,
}

impl Default for GameItem {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            prop_food: None,
            prop_armor: None,
            prop_block: None,
            prop_tool: None,
        }
    }
}
