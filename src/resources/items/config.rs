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

impl ItemConfig {
    pub fn id(&self) -> &String {
        &self.item.id
    }
}
