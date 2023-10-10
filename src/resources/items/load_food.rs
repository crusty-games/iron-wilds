use super::config::{Consumable, Destructible, Item, Placable};

pub fn load_food_items() -> Vec<Item> {
    let mut items: Vec<Item> = vec![];

    items.push(Item {
        id: "bread".into(),
        name: "Bread".into(),
        consumable: Some(Consumable {
            effect_healing: 20.0,
        }),
        ..Default::default()
    });

    items.push(Item {
        id: "cake".into(),
        name: "Cake".into(),
        consumable: Some(Consumable {
            effect_healing: 50.0,
        }),
        placable: Some(Placable),
        destructible: Some(Destructible),
        ..Default::default()
    });

    items
}
