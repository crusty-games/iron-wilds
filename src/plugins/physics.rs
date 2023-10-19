use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};

use crate::components::physics::{GravitateToPlayer, Physics};
use crate::components::player::Player;
use crate::resources::physics::PhysicsTimer;
use crate::systems::physics::{compute_physics, gravitate, update_physics_shapes};
use crate::systems::physics::{tick_physics_timer, PhysicsSet};

pub struct IronWildsPhysicsPlugin;
impl Plugin for IronWildsPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsTimer>()
            .register_type::<Physics>()
            .add_systems(Update, tick_physics_timer)
            .add_systems(
                Update,
                (gravitate::<GravitateToPlayer, Player>, compute_physics)
                    .chain()
                    .in_set(PhysicsSet::Computation),
            )
            .add_systems(
                Update,
                (update_physics_shapes)
                    .chain()
                    .after(PhysicsSet::Computation)
                    .in_set(PhysicsSet::Rendering),
            );
    }
}
