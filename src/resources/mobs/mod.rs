pub mod config;

use std::collections::HashMap;

use bevy::prelude::*;

use self::config::{AssetConfig, MobConfig, MovementConfig};

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
                movement: MovementConfig::RandomWalk {
                    speed: 20.0,
                    idle_secs: 1.0..5.0,
                },
                assets: Some(AssetConfig {
                    test_path: "test/ghost.png".into(),
                }),
            },
        );

        Self { mobs }
    }
}

impl MobStore {
    pub fn get<S: AsRef<str>>(&self, id: S) -> &MobConfig {
        let id_ref = id.as_ref();
        match self.mobs.get(id_ref) {
            Some(mob) => mob,
            None => panic!("Mob by ID \"{id_ref}\" not found"),
        }
    }
}
