use crate::components::items::{Item, Weapon};

use super::config::ItemConfig;

pub fn load_weapon_items() -> Vec<ItemConfig> {
    let mut items: Vec<ItemConfig> = vec![];

    items.push(ItemConfig {
        item: Item {
            id: "sword".into(),
            name: "Sword".into(),
        },
        weapon: Some(Weapon { base_damage: 20.0 }),
        ..Default::default()
    });

    items
}
