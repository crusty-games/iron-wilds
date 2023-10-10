pub mod config;
pub mod load;
pub mod store;

use bevy::prelude::*;

use self::store::{ItemStore, ITEM_STORE};

#[derive(Resource)]
pub struct Items {
    pub store: &'static ItemStore,
}

impl Default for Items {
    fn default() -> Self {
        Self { store: &ITEM_STORE }
    }
}
