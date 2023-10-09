use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, ShapeBundle};

use super::physics::Physics;

#[derive(Component, Clone, Default)]
pub struct Item {
    pub id: String,
    pub name: String,
}

impl Item {
    pub fn new<S: AsRef<str>>(id: S, name: S) -> Self {
        Self {
            id: id.as_ref().into(),
            name: name.as_ref().into(),
        }
    }
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
pub struct Tool;

#[derive(Component, Clone, Default)]
pub struct Weapon {
    pub base_damage: f32,
}

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

#[derive(Component, Clone, Default)]
pub struct BlockItem;

#[derive(Component, Clone, Default)]
pub struct InventoryItem {
    pub stack_count: usize,
}

// Bundles
#[derive(Bundle)]
pub struct GroundItemBundle {
    pub ground_item: GroundItem,
    pub physics: Physics,
    pub shape: ShapeBundle,
    pub fill: Fill,
}
