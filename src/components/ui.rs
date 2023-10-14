use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryRoot;
#[derive(Component)]
pub struct InventoryContainer;
#[derive(Component)]
pub struct InventorySlot {
    pub slot_index: usize,
}
