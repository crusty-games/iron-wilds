use crate::components::items::{Consumable, Destructible, Item, Placable};

use super::config::ItemConfig;

pub fn load_food_items() -> Vec<ItemConfig> {
    let mut items: Vec<ItemConfig> = vec![];

    items.push(ItemConfig {
        item: Item {
            id: "bread".into(),
            name: "Bread".into(),
        },
        consumable: Some(Consumable {
            effect_healing: 20.0,
        }),
        ..Default::default()
    });

    items.push(ItemConfig {
        item: Item {
            id: "cake".into(),
            name: "Cake".into(),
        },
        consumable: Some(Consumable {
            effect_healing: 50.0,
        }),
        placable: Some(Placable::default()),
        destructible: Some(Destructible),
        ..Default::default()
    });

    items
}
