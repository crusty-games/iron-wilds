#[derive(Clone)]
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

impl Default for ItemConfig {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            consumable: None,
            stackable: None,
            placable: None,
            destructible: None,
            harvestable: None,
            tool: None,
            weapon: None,
        }
    }
}

// Food Related
#[derive(Clone, Default)]
pub struct Placable {
    pub is_wall: bool,
    pub is_floor: bool,
}

#[derive(Clone, Default)]
pub struct Consumable {
    pub effect_healing: f32,
}

// Tool/Weapon Related
#[derive(Clone)]
pub struct Tool;

#[derive(Clone, Default)]
pub struct Weapon {
    pub base_damage: f32,
}

// Block Related
#[derive(Clone)]
pub struct Destructible;

#[derive(Clone)]
pub struct Harvestable;

// General item data
#[derive(Clone, Default)]
pub struct Stackable {
    pub max_stack: usize,
}
