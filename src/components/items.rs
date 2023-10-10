use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_prototype_lyon::prelude::{Fill, ShapeBundle};

use super::physics::{Gravitate, GravitateToPlayer, Physics};

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct GroundItem {
    pub item_id: String,
    pub stack_count: usize,
}

#[derive(Bundle)]
pub struct GroundItemBundle {
    pub ground_item: GroundItem,
    pub gravitate: (Gravitate, GravitateToPlayer),
    pub physics: Physics,
    pub shape: ShapeBundle,
    pub fill: Fill,
}
