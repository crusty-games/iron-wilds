use bevy::prelude::*;

#[derive(Event)]
pub struct ActiveSlotChangeEvent {
    pub slot_index: usize,
}
