use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::physics::Physics;

pub struct IronWildsPlayerPlugin;
impl Plugin for IronWildsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
}

fn spawn_player(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 20.0,
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::WHITE),
        Player {
            movement_speed: 3.0,
        },
        Physics {
            velocity: Vec2 { x: 0.0, y: 10.0 },
            ..default()
        },
    ));
}
