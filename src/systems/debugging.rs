use bevy::prelude::*;

use crate::components::physics::Physics;

pub fn debug_physics(physics_query: Query<&Physics>, mut gizmos: Gizmos) {
    for particle in physics_query.iter() {
        gizmos.circle_2d(particle.position, 0.1, Color::RED);
        gizmos.line_2d(
            particle.position,
            particle.position + particle.velocity * 5.0,
            Color::BLUE,
        );
    }
}
