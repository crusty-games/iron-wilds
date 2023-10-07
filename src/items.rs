use bevy::prelude::*;

use crate::bevy_component;

// Item and Modifiers
bevy_component!(Item);
bevy_component!(ItemGroundLoot);

// Item Properties
bevy_component!(ItemFood);
bevy_component!(ItemArmor);
bevy_component!(ItemBlock);
bevy_component!(ItemTool);
