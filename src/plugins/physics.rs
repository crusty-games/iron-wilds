use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};

use crate::components::physics::{GravitateToPlayer, Physics};
use crate::components::player::Player;
use crate::resources::physics::PhysicsTimer;
use crate::systems::physics::{
    compute_physics, gravitate, tick_physics_timer, update_physics_shapes,
};

pub struct IronWildsPhysicsPlugin;
impl Plugin for IronWildsPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsTimer>()
            .register_type::<Physics>()
            .add_systems(Update, tick_physics_timer)
            .add_systems(
                Update,
                (
                    gravitate::<GravitateToPlayer, Player>,
                    compute_physics,
                    update_physics_shapes,
                )
                    .chain(),
            );
    }
}
