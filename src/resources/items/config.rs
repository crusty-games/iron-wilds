use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, ShapeBundle},
    shapes::Circle,
};
use rand::random;

use crate::components::{
    items::{
        Consumable, Destructible, GroundItem, GroundItemBundle, Harvestable, Item, Placable,
        Stackable, Tool, Weapon,
    },
    physics::Physics,
};

#[derive(Clone)]
pub struct ItemConfig {
    pub item: Item,
    pub consumable: Option<Consumable>,
    pub stackable: Option<Stackable>,
    pub placable: Option<Placable>,
    pub destructible: Option<Destructible>,
    pub harvestable: Option<Harvestable>,
    pub tool: Option<Tool>,
    pub weapon: Option<Weapon>,
}

impl Default for ItemConfig {
    fn default() -> Self {
        Self {
            item: Default::default(),
            consumable: None,
            stackable: None,
            placable: None,
            destructible: None,
            harvestable: None,
            tool: None,
            weapon: None,
        }
    }
}

macro_rules! add_component {
    ($target:expr, $from:expr) => {
        if let Some(component) = $from.clone() {
            $target.insert(component);
        }
    };
}

impl ItemConfig {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        let mut entity_commands = commands.spawn(self.item.clone());
        entity_commands.insert(Name::from(self.item.name.clone()));
        add_component!(entity_commands, self.consumable);
        add_component!(entity_commands, self.stackable);
        add_component!(entity_commands, self.placable);
        add_component!(entity_commands, self.destructible);
        add_component!(entity_commands, self.harvestable);
        add_component!(entity_commands, self.tool);
        add_component!(entity_commands, self.weapon);
        entity_commands.id()
    }

    pub fn spawn_as_ground_item(&self, commands: &mut Commands, position: Vec2) -> Entity {
        let entity = self.spawn(commands);
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(GroundItemBundle {
            ground_item: GroundItem::default(),
            physics: Physics {
                position,
                velocity: Vec2 {
                    x: random::<f32>() - 0.5,
                    y: random::<f32>() - 0.5,
                }
                .normalize()
                    * 10.0,
                friction: 0.8,
            },
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&Circle {
                    radius: 5.0,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            fill: Fill::color(Color::RED),
        });
        entity_commands.id()
    }
}
