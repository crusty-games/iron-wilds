use bevy::prelude::*;

use crate::components::items::{
    Consumable, Destructible, Harvestable, Item, Placable, Stackable, Tool, Weapon,
};

#[derive(Clone)]
pub struct ItemConfig {
    pub item: Item,
    pub consumable: Option<Consumable>,
    pub stackable: Option<Stackable>,
    pub placable: Option<Placable>,
    pub destructible: Option<Destructible>,
    pub harvestable: Option<Harvestable>,
    pub tool: Option<Tool>,
    pub weapon: Option<Weapon>,
}

impl Default for ItemConfig {
    fn default() -> Self {
        Self {
            item: Default::default(),
            consumable: None,
            stackable: None,
            placable: None,
            destructible: None,
            harvestable: None,
            tool: None,
            weapon: None,
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
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        let mut entity = commands.spawn(self.item.clone());
        entity.insert(Name::from(self.item.name.clone()));
        add_component!(entity, self.consumable);
        add_component!(entity, self.stackable);
        add_component!(entity, self.placable);
        add_component!(entity, self.destructible);
        add_component!(entity, self.harvestable);
        add_component!(entity, self.tool);
        add_component!(entity, self.weapon);
        entity.id()
    }
}