use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use super::physics::{Gravitate, GravitateToPlayer, Physics};

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct GroundItem {
    pub item_id: String,
    pub stack_count: usize,
    pub pick_up_timeout: Timer,
}

#[derive(Bundle)]
pub struct GroundItemBundle {
    pub ground_item: GroundItem,
    pub gravitate: (Gravitate, GravitateToPlayer),
    pub physics: Physics,
}
