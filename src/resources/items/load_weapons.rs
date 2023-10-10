use super::config::{Item, Weapon};

pub fn load_weapon_items() -> Vec<Item> {
    let mut items: Vec<Item> = vec![];

    items.push(Item {
        id: "sword".into(),
        name: "Sword".into(),
        weapon: Some(Weapon { base_damage: 20.0 }),
        ..Default::default()
    });

    items
}
