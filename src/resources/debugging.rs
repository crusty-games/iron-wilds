use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct DebugSettings {
    debug_physics: bool,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            debug_physics: true,
        }
    }
}
