use bevy::prelude::*;
use uuid::Uuid;
use crate::game::components::ChipType;

#[derive(Event)]
pub struct ChipPlacedEvent {
    pub chip_type: ChipType,
    pub translation: Vec3,
    pub uuid: Uuid,
}

impl ChipPlacedEvent {
    pub fn new(chip_type: ChipType, translation: Vec3, uuid: Uuid) -> Self{
        Self {
            chip_type,
            translation,
            uuid
        }
    }
}