use bevy::prelude::*;
use uuid::Uuid;
use crate::game::components::{ChipType, Chip};
use std::sync::{Arc, Mutex};

#[derive(Event)]
pub struct ChipPlacedEvent {
    pub chip_ref: Arc<Mutex<Chip>>,
    pub chip_type: ChipType,
    pub translation: Vec3,
    pub uuid: Uuid,
}

impl ChipPlacedEvent {
    pub fn new(chip_ref: Arc<Mutex<Chip>>, chip_type: ChipType, translation: Vec3, uuid: Uuid) -> Self{
        Self {
            chip_ref,
            chip_type,
            translation,
            uuid
        }
    }
}