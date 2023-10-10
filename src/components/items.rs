use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, ShapeBundle};

use super::physics::Physics;

// Item State Modifer
#[derive(Component, Clone, Default)]
pub struct GroundItem {
    pub item_id: String,
    pub count: usize,
}

// Bundles
#[derive(Bundle)]
pub struct GroundItemBundle {
    pub ground_item: GroundItem,
    pub physics: Physics,
    pub shape: ShapeBundle,
    pub fill: Fill,
}
