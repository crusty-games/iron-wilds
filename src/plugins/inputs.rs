use bevy::prelude::*;

use crate::systems::inputs::move_player;
use crate::systems::physics::compute_physics;

pub struct IronWildsInputsPlugin;
impl Plugin for IronWildsInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player.before(compute_physics));
    }
}
