use super::config::{
    AssetConfig, ConsumableConfig, DestructibleConfig, ItemConfig, ItemDropConfig, PlacableConfig,
    WeaponConfig,
};

pub fn load_sample_items() -> Vec<ItemConfig> {
    vec![
        ItemConfig {
            id: "crab".into(),
            name: "Crab".into(),
            max_stack_count: 12,
            consumable: Some(ConsumableConfig {
                effect_healing: 20.0,
            }),
            assets: Some(AssetConfig {
                ground_item_path: "test/crab.png".into(),
            }),
            ..Default::default()
        },
        ItemConfig {
            id: "anvil".into(),
            name: "Anvil".into(),
            max_stack_count: 4,
            placable: Some(PlacableConfig {}),
            destructible: Some(DestructibleConfig {
                drops: vec![ItemDropConfig {
                    item_id: "anvil".into(),
                    stack_count: 1..1,
                    chance: 1.0,
                }],
            }),
            assets: Some(AssetConfig {
                ground_item_path: "test/anvil.png".into(),
            }),
            ..Default::default()
        },
        ItemConfig {
            id: "sword".into(),
            name: "Sword".into(),
            weapon: Some(WeaponConfig { base_damage: 20.0 }),
            assets: Some(AssetConfig {
                ground_item_path: "test/sword.png".into(),
            }),
            ..Default::default()
        },
    ]
}
