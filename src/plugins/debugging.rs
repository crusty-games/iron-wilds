use bevy::prelude::*;

use crate::{resources::debugging::DebugSettings, systems::debugging::debug_physics};

pub struct IronWildsDebuggingPlugin;
impl Plugin for IronWildsDebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DebugSettings>()
            .init_resource::<DebugSettings>()
            .add_systems(Update, debug_physics);
    }
}
