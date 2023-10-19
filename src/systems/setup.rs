use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes::{Rectangle, RectangleOrigin},
};

pub fn spawn_camera(mut commands: Commands) {
    let scale = 1.0 / 48.0;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 {
            x: scale,
            y: scale,
            z: scale,
        }),
        ..default()
    });
}

pub fn place_unit_tile(mut commands: Commands) {
    commands.spawn((
        Name::new("UnitTile"),
        ShapeBundle {
            path: GeometryBuilder::build_as(&Rectangle {
                extents: Vec2 { x: 1.0, y: 1.0 },
                origin: RectangleOrigin::BottomLeft, // Y+ Up, X+ Right
            }),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
        Fill::color(Color::WHITE),
    ));
}

pub fn say_hello() {
    println!(
        "\
░▀█▀░█▀▄░█▀█░█▀█    
░░█░░█▀▄░█░█░█░█    
░▀▀▀░▀░▀░▀▀▀░▀░▀    
░█░█░▀█▀░█░░░█▀▄░█▀▀
░█▄█░░█░░█░░░█░█░▀▀█
░▀░▀░▀▀▀░▀▀▀░▀▀░░▀▀▀"
    );
}
