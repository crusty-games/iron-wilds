use std::ops::Range;

pub struct ItemConfig {
    pub id: String,
    pub name: String,
    pub max_stack_count: usize,

    pub consumable: Option<Consumable>,
    pub placable: Option<Placable>,
    pub destructible: Option<Destructible>,
    pub harvestable: Option<Harvestable>,
    pub tool: Option<Tool>,
    pub weapon: Option<Weapon>,

    pub assets: Option<AssetConfig>,
}

impl Default for ItemConfig {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            max_stack_count: 1,
            consumable: None,
            placable: None,
            destructible: None,
            harvestable: None,
            tool: None,
            weapon: None,
            assets: None,
        }
    }
}
pub struct AssetConfig {
    pub ground_item_path: String,
    // pub tile_path: String,
}
pub struct Consumable {
    pub effect_healing: f32,
}

pub struct Tool;
pub struct Weapon {
    pub base_damage: f32,
}
pub struct Placable {}
pub struct BlockItemDrop {
    pub item_id: String,
    pub stack_count: Range<usize>,
    pub chance: f32,
}
pub struct Destructible {
    pub drops: Vec<BlockItemDrop>,
}
pub struct Harvestable;
