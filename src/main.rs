mod inputs;
mod items;
mod logic;
mod physics;
mod player;
mod setup;

use inputs::IronWildsInputsPlugin;
use items::IronWildsItemsPlugin;
use physics::IronWildsPhysicsPlugin;
use player::IronWildsPlayerPlugin;
use setup::IronWildsSetupPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(IronWildsSetupPlugin)
        .add_plugins(IronWildsItemsPlugin)
        .add_plugins(IronWildsPhysicsPlugin)
        .add_plugins(IronWildsPlayerPlugin)
        .add_plugins(IronWildsInputsPlugin)
        .run();
}
