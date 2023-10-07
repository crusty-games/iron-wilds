```rs
use bevy::prelude::*;

pub struct IronWildsTemplatePlugin;
impl Plugin for IronWildsTemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_world);
    }
}

fn hello_world() {
    println!("Welcome to Iron Wilds!");
}
```