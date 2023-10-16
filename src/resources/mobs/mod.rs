pub mod config;

use std::collections::HashMap;

use bevy::prelude::*;

use self::config::{MobConfig, MovementConfig};

#[derive(Resource)]
pub struct MobStore {
    pub mobs: HashMap<String, MobConfig>,
}

impl Default for MobStore {
    fn default() -> Self {
        let mut mobs = HashMap::new();

        mobs.insert(
            "ghost".into(),
            MobConfig {
                id: "ghost".into(),
                name: "Ghost".into(),
                max_health: 100.0,
                damage: None,
                movement: Some(MovementConfig::RandomWalk {
                    speed: 20.0,
                    interval_ms: 1000.0..5000.0,
                }),
            },
        );

        Self { mobs }
    }
}
