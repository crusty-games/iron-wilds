pub mod components;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod systems;

#[cfg(test)]
mod tests;

use plugins::inputs::IronWildsInputsPlugin;
use plugins::inventory::IronWildsInventoryPlugin;
use plugins::items::IronWildsItemsPlugin;
use plugins::physics::IronWildsPhysicsPlugin;
use plugins::player::IronWildsPlayerPlugin;
use plugins::setup::IronWildsSetupPlugin;

use bevy::prelude::*;
use plugins::ui::IronWildsUiPlugin;

fn main() {
    App::new()
        .add_plugins(IronWildsSetupPlugin)
        .add_plugins(IronWildsItemsPlugin)
        .add_plugins(IronWildsInventoryPlugin)
        .add_plugins(IronWildsPhysicsPlugin)
        .add_plugins(IronWildsPlayerPlugin)
        .add_plugins(IronWildsInputsPlugin)
        .add_plugins(IronWildsUiPlugin)
        .run();
}
