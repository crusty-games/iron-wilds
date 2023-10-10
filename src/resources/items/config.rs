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

// Asset Related
pub struct AssetConfig {
    pub ground_item_path: String,
}

// Food Related
pub struct Placable;
pub struct Consumable {
    pub effect_healing: f32,
}

// Tool/Weapon Related
pub struct Tool;
pub struct Weapon {
    pub base_damage: f32,
}

// Block Related
pub struct Destructible;
pub struct Harvestable;
