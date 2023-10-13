use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::components::storage::Storage;

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
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
