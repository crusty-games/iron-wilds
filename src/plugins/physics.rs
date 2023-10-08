use bevy::prelude::{App, Plugin, Update};

use crate::resources::physics::PhysicsTimer;
use crate::systems::physics::{compute_physics, tick_physics_timer};

pub struct IronWildsPhysicsPlugin;
impl Plugin for IronWildsPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsTimer>()
            .add_systems(Update, tick_physics_timer)
            .add_systems(Update, compute_physics);
    }
}