use super::config::{
    AssetConfig, BlockItemDrop, Consumable, Destructible, ItemConfig, Placable, Weapon,
};

pub fn load_sample_items() -> Vec<ItemConfig> {
    let mut items: Vec<ItemConfig> = vec![];

    items.push(ItemConfig {
        id: "crab".into(),
        name: "Crab".into(),
        max_stack_count: 12,
        consumable: Some(Consumable {
            effect_healing: 20.0,
        }),
        assets: Some(AssetConfig {
            ground_item_path: "test/crab.png".into(),
        }),
        ..Default::default()
    });

    items.push(ItemConfig {
        id: "anvil".into(),
        name: "Anvil".into(),
        max_stack_count: 4,
        placable: Some(Placable {}),
        destructible: Some(Destructible {
            drops: vec![BlockItemDrop {
                item_id: "anvil".into(),
                stack_count: 1..1,
                chance: 1.0,
            }],
        }),
        assets: Some(AssetConfig {
            ground_item_path: "test/anvil.png".into(),
        }),
        ..Default::default()
    });

    items.push(ItemConfig {
        id: "sword".into(),
        name: "Sword".into(),
        weapon: Some(Weapon { base_damage: 20.0 }),
        assets: Some(AssetConfig {
            ground_item_path: "test/sword.png".into(),
        }),
        ..Default::default()
    });

    items
}
