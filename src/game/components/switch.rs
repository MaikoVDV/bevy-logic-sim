use bevy::prelude::*;
use super::*;

#[derive(Component)]
pub struct ManualSwitch {
    pub state: bool,
    pub output_pins: Vec<Pin>
}

impl ManualSwitch {
    pub fn new(output_pins: Option<Vec<Pin>>) -> Self {
        Self {
            state: false,
            output_pins: match output_pins {
                Some(pins) => pins,
                None => Vec::new()
            }
        }
    }
    
    pub fn toggle(&mut self) {
    }
}

pub fn graphics_manager(
    mut switch_query: Query<(&mut Sprite, &ManualSwitch)>
) {
    for (mut sprite, switch) in switch_query.iter_mut() {
        sprite.color = if switch.state { Color::RED } else {Color::DARK_GRAY };
    }
}