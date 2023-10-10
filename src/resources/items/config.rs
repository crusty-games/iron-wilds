#[derive(Default)]
pub struct ItemConfig {
    pub id: String,
    pub name: String,

    pub consumable: Option<Consumable>,
    pub stackable: Option<Stackable>,
    pub placable: Option<Placable>,
    pub destructible: Option<Destructible>,
    pub harvestable: Option<Harvestable>,
    pub tool: Option<Tool>,
    pub weapon: Option<Weapon>,
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

// General item data
pub struct Stackable {
    pub max_stack: usize,
}
