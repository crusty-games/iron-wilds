mod load;
pub mod store;

#[derive(Default)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub max_stack: usize,

    pub consumable: Option<Consumable>,
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
