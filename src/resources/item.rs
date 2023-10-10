use bevy::prelude::*;

use crate::game::items::store::{ItemStore, ITEM_STORE};

#[derive(Resource)]
pub struct Items {
    pub store: &'static ItemStore,
}

impl Default for Items {
    fn default() -> Self {
        Self { store: &ITEM_STORE }
    }
}
