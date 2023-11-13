use std::ops::Range;

use crate::resources::items::config::ItemDropConfig;

#[derive(Default)]
pub struct MobConfig {
    pub id: String,
    pub name: String,
    pub max_health: f32,

    pub damage: Option<DamageConfig>,
    pub movement: MovementConfig,

    pub drops: Vec<ItemDropConfig>,
    pub assets: Option<AssetConfig>,
}

pub struct AssetConfig {
    pub test_path: String,
}

pub struct DamageConfig {
    pub effect: f32,
}

#[derive(Default)]
pub enum MovementConfig {
    #[default]
    None,
    RandomWalk {
        speed: f32,
        idle_secs: Range<f32>,
        walk_radius: f32,
    },
}
