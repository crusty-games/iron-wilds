use std::ops::Range;

#[derive(Default)]
pub struct MobConfig {
    pub id: String,
    pub name: String,
    pub max_health: f32,

    pub damage: Option<DamageConfig>,
    pub movement: MovementConfig,

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
    },
}
