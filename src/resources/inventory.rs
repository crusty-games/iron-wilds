use bevy::prelude::*;

use crate::components::storage::Storage;

#[derive(Resource)]
pub struct Inventory {
    pub storage: Storage,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            storage: Storage::new(8),
        }
    }
}
