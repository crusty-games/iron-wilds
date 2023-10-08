use bevy::prelude::*;

type PlayerId = usize;

#[derive(Component, Clone, Default)]
pub struct Item {
    pub id: String,
    pub name: String,
}

// Food Related
#[derive(Component, Clone, Default)]
pub struct Placable {
    pub is_wall: bool,
    pub is_floor: bool,
}

#[derive(Component, Clone, Default)]
pub struct Consumable {
    pub effect_healing: f32,
}

// Tool/Weapon Related
#[derive(Component, Clone)]
pub struct Usable;

// Block Related
#[derive(Component, Clone)]
pub struct Destructible;

#[derive(Component, Clone)]
pub struct Harvestable;

// General item data
#[derive(Component, Clone, Default)]
pub struct Stackable {
    pub max_stack: usize,
}

// Item State Modifer
#[derive(Component, Clone, Default)]
pub struct GroundItem {
    pub count: usize,
}

#[derive(Component, Clone)]
pub struct BlockItem;

#[derive(Component, Clone, Default)]
pub struct InventoryItem {
    pub player_id: PlayerId,
    pub stack_count: usize,
}
