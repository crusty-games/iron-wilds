use std::ops::Range;

pub struct ItemConfig {
    pub id: String,
    pub name: String,
    pub max_stack_count: usize,

    pub consumable: Option<ConsumableConfig>,
    pub placable: Option<PlacableConfig>,
    pub destructible: Option<DestructibleConfig>,
    pub harvestable: Option<HarvestableConfig>,
    pub tool: Option<ToolConfig>,
    pub weapon: Option<WeaponConfig>,

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
pub struct ConsumableConfig {
    pub effect_healing: f32,
}

pub struct ToolConfig;
pub struct WeaponConfig {
    pub base_damage: f32,
}
pub struct PlacableConfig {}
pub struct ItemDropConfig {
    pub item_id: String,
    pub stack_count: Range<usize>,
    pub chance: f32,
}
pub struct DestructibleConfig {
    pub drops: Vec<ItemDropConfig>,
}
pub struct HarvestableConfig {
    pub drops: Vec<ItemDropConfig>,
}
