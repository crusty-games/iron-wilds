use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{Fill, ShapeBundle};

use super::physics::{Gravitate, GravitateToPlayer, Physics};

// Item State Modifer
#[derive(Component)]
pub struct GroundItem {
    pub item_id: String,
    pub stack_count: usize,
}

// Bundles
#[derive(Bundle)]
pub struct GroundItemBundle {
    pub ground_item: GroundItem,
    pub gravitate: (Gravitate, GravitateToPlayer),
    pub physics: Physics,
    pub shape: ShapeBundle,
    pub fill: Fill,
}
