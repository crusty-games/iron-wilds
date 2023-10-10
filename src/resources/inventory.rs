use bevy::prelude::*;

use crate::components::storage::Storage;

#[derive(Resource)]
pub struct PrimaryPlayerInventory<'a> {
    pub storage: Storage<'a>,
}

impl Default for PrimaryPlayerInventory<'_> {
    fn default() -> Self {
        Self {
            storage: Storage::new(8),
        }
    }
}
