use std::sync::{Mutex, Arc};

use bevy::prelude::*;

use super::Chip;

#[derive(Component)]
pub struct ChipRef(pub Arc<Mutex<Chip>>);