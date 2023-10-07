use bevy::prelude::*;

pub struct IronWildsItemsPlugin;
impl Plugin for IronWildsItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, items_system);
    }
}

fn items_system() {
    println!("Items system not yet implemented!");
}
