use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component)]
pub struct ChipRef(pub Uuid);