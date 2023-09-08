use bevy::prelude::*;
use crate::game::Chip;

#[derive(Event)]
pub struct ChipPlacedEvent {
    pub chip: Chip,
    pub translation: Vec3
}

impl ChipPlacedEvent {
    pub fn new(chip: Chip, translation: Vec3) -> Self{
        Self {
            chip,
            translation
        }
    }
}