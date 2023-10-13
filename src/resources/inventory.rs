use std::ops::Range;

use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::components::storage::Storage;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Inventory {
    pub storage: Storage,
    pub hotbar: Hotbar,
}

pub const INVENTORY_COLUMNS: usize = 8;
pub const INVENTORY_ROWS: usize = 1;

impl Default for Inventory {
    fn default() -> Self {
        Self {
            storage: Storage::new(INVENTORY_COLUMNS * INVENTORY_ROWS),
            hotbar: Hotbar {
                active_slot: 0,
                capacity: INVENTORY_COLUMNS,
            },
        }
    }
}

#[derive(Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Hotbar {
    pub active_slot: usize,
    pub capacity: usize,
}

impl Hotbar {
    pub fn range(&self) -> Range<usize> {
        0..self.capacity
    }
}
