mod inputs;
mod items;
mod logic;
mod physics;
mod player;
mod setup;
mod utils;

use inputs::IronWildsInputsPlugin;
use physics::IronWildsPhysicsPlugin;
use player::IronWildsPlayerPlugin;
use setup::IronWildsSetupPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(IronWildsSetupPlugin)
        .add_plugins(IronWildsPhysicsPlugin)
        .add_plugins(IronWildsPlayerPlugin)
        .add_plugins(IronWildsInputsPlugin)
        .run();
}
