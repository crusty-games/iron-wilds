use bevy::prelude::*;

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
