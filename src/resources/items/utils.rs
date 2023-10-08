use bevy::prelude::*;

use crate::components::items::{
    Consumable, Destructible, Harvestable, Item, Placable, Stackable, Usable,
};

#[derive(Clone)]
pub struct ItemConfig {
    pub item: Item,
    pub consumable: Option<Consumable>,
    pub stackable: Option<Stackable>,
    pub placable: Option<Placable>,
    pub usable: Option<Usable>,
    pub destructible: Option<Destructible>,
    pub harvestable: Option<Harvestable>,
}

impl Default for ItemConfig {
    fn default() -> Self {
        Self {
            item: Default::default(),
            consumable: None,
            stackable: None,
            placable: None,
            usable: None,
            destructible: None,
            harvestable: None,
        }
    }
}

macro_rules! add_component {
    ($target:expr, $from:expr) => {
        if let Some(component) = $from.clone() {
            $target.insert(component);
        }
    };
}

impl ItemConfig {
    pub fn spawn(&self, commands: &mut Commands) {
        let mut entity = commands.spawn(self.item.clone());
        add_component!(entity, self.consumable);
        add_component!(entity, self.stackable);
        add_component!(entity, self.placable);
        add_component!(entity, self.usable);
        add_component!(entity, self.destructible);
        add_component!(entity, self.harvestable);
    }
}
