use super::{Consumable, Destructible, Item, Placable, Weapon};

pub fn load_sample_items() -> Vec<Item> {
    let mut items: Vec<Item> = vec![];

    items.push(Item {
        id: "bread".into(),
        name: "Bread".into(),
        max_stack: 12,
        consumable: Some(Consumable {
            effect_healing: 20.0,
        }),
        ..Default::default()
    });

    items.push(Item {
        id: "cake".into(),
        name: "Cake".into(),
        max_stack: 1,
        consumable: Some(Consumable {
            effect_healing: 50.0,
        }),
        placable: Some(Placable),
        destructible: Some(Destructible),
        ..Default::default()
    });

    items.push(Item {
        id: "sword".into(),
        name: "Sword".into(),
        max_stack: 1,
        weapon: Some(Weapon { base_damage: 20.0 }),
        ..Default::default()
    });

    items
}
