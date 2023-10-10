use super::config::{Consumable, Destructible, ItemConfig, Placable, Weapon};

pub fn load_sample_items() -> Vec<ItemConfig> {
    let mut items: Vec<ItemConfig> = vec![];

    items.push(ItemConfig {
        id: "bread".into(),
        name: "Bread".into(),
        max_stack_count: 12,
        consumable: Some(Consumable {
            effect_healing: 20.0,
        }),
        ..Default::default()
    });

    items.push(ItemConfig {
        id: "cake".into(),
        name: "Cake".into(),
        max_stack_count: 4,
        consumable: Some(Consumable {
            effect_healing: 50.0,
        }),
        placable: Some(Placable),
        destructible: Some(Destructible),
        ..Default::default()
    });

    items.push(ItemConfig {
        id: "sword".into(),
        name: "Sword".into(),
        max_stack_count: 1,
        weapon: Some(Weapon { base_damage: 20.0 }),
        ..Default::default()
    });

    items
}
