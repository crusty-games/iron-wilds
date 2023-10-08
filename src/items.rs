use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes::Circle,
};
use rand::random;

use crate::{
    logic::items::{store::GAME_ITEM_STORE, GameItem},
    physics::Physics,
};

// Item and Modifiers
#[derive(Component)]
pub struct Item {
    pub game_item: GameItem,
    pub count: usize,
}

#[derive(Component)]
pub struct ItemGroundLoot {
    // pub previous_owner: Option<PlayerId>,
}

#[derive(Bundle)]
pub struct ItemGroundLootBundle {
    ground_loot: ItemGroundLoot,
    physics: Physics,
    shape: ShapeBundle,
    fill: Fill,
}

pub struct IronWildsItemsPlugin;
impl Plugin for IronWildsItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_items);
    }
}

fn create_ground_loot(game_item: GameItem, count: usize, position: Vec2) -> impl Bundle {
    (
        Item { game_item, count },
        ItemGroundLootBundle {
            ground_loot: ItemGroundLoot {},
            physics: Physics {
                position,
                velocity: Vec2 {
                    x: random::<f32>() - 0.5,
                    y: random::<f32>() - 0.5,
                }
                .normalize()
                    * 10.0,
                ..default()
            },
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&Circle {
                    radius: 5.0,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            fill: Fill::color(Color::RED),
        },
    )
}

fn spawn_items(mut commands: Commands) {
    let game_item = GAME_ITEM_STORE.get_by_id("bread".into());
    commands.spawn(create_ground_loot(game_item.clone(), 1, Vec2::ZERO));
    commands.spawn(create_ground_loot(game_item.clone(), 1, Vec2::ZERO));
    commands.spawn(create_ground_loot(game_item.clone(), 1, Vec2::ZERO));
}
