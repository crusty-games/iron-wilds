use crate::physics::Physics;

use super::{game_items::GAME_ITEM_LOADER, GameItem, InventoryItem};
use bevy::prelude::*;
use bevy_inspector_egui::egui::Stroke;
use bevy_prototype_lyon::prelude::*;
use rand::random;

pub struct IronWildsItemsPlugin;
impl Plugin for IronWildsItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground_loot);
    }
}

#[derive(Component)]
pub struct GroundLoot {
    pub game_item: GameItem,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    pub capacity: u16,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            capacity: 8,
        }
    }
}

fn spawn_ground_loot(mut commands: Commands) {
    for _ in 0..10 {
        let x = random::<f32>() * 1000.0;
        let y = random::<f32>() * 1000.0;
        commands.spawn((
            GroundLoot {
                game_item: GAME_ITEM_LOADER.get_by_id("iron".into()).clone(),
            },
            Physics::default(),
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 5.0,
                    ..default()
                }),
                transform: Transform::from_xyz(x, y, 1.0),
                ..default()
            },
            Fill::color(Color::RED),
        ));
    }
}
