use std::ops::Range;

#[derive(Default)]
pub struct MobConfig {
    pub id: String,
    pub name: String,
    pub max_health: f32,

    pub damage: Option<DamageConfig>,
    pub movement: Option<MovementConfig>,
}

pub struct DamageConfig {
    pub effect: f32,
}

pub enum MovementConfig {
    RandomWalk { speed: f32, interval_ms: Range<f32> },
}
