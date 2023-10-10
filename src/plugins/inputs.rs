use bevy::prelude::*;

use crate::systems::inputs::{drop_items, move_player};

pub struct IronWildsInputsPlugin;
impl Plugin for IronWildsInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_player, drop_items));
    }
}
