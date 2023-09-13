use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct ChipClicked(Entity);

impl From<ListenerInput<Pointer<Click>>> for ChipClicked {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        ChipClicked(event.target)
    }
}