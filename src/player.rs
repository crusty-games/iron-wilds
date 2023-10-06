use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::physics::{Physics, PhysicsTimer};

pub struct IronWildsPlayerPlugin;
impl Plugin for IronWildsPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, update_player_shape);
    }
}

#[derive(Component)]
pub struct Player {
    pub aim_direction: f32,
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
        Player { aim_direction: 0.0 },
        Physics {
            velocity: Vec2 { x: 0.0, y: 10.0 },
            ..default()
        },
    ));
}

fn update_player_shape(
    mut player_query: Query<(&Physics, &mut Transform), With<Player>>,
    physics_timer: Res<PhysicsTimer>,
) {
    let perc_left = physics_timer.timer.percent();
    for (player_physics, mut transform) in player_query.iter_mut() {
        let translation = player_physics.position + (player_physics.velocity * perc_left);

        transform.translation = Vec3 {
            x: translation.x,
            y: translation.y,
            z: 0.0,
        }
    }
}
