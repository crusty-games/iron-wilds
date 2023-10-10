mod components;
mod events;
mod game;
mod plugins;
mod resources;
mod systems;

use plugins::inputs::IronWildsInputsPlugin;
use plugins::items::IronWildsItemsPlugin;
use plugins::physics::IronWildsPhysicsPlugin;
use plugins::player::IronWildsPlayerPlugin;
use plugins::setup::IronWildsSetupPlugin;

use bevy::prelude::*;

#[macro_use]
extern crate lazy_static;

fn main() {
    App::new()
        .add_plugins(IronWildsSetupPlugin)
        .add_plugins(IronWildsItemsPlugin)
        .add_plugins(IronWildsPhysicsPlugin)
        .add_plugins(IronWildsPlayerPlugin)
        .add_plugins(IronWildsInputsPlugin)
        .run();
}
