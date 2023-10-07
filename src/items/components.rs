use bevy::prelude::*;

#[derive(Component)]
pub struct Item {}

#[derive(Component)]
pub struct GroundLoot;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: u16,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 8,
        }
    }
}
