use bevy::prelude::*;
use super::*;

#[derive(Component)]
pub struct Light {
    pub state: bool,
    pub input_pin: Pin
}

impl Light {
    pub fn new() -> Self {
        Self {
            state: false,
            input_pin: Pin::new()
        }
    }
    
    pub fn _toggle(&mut self) {
        self.state = !self.state;
    }
    pub fn set(&mut self, new_state: bool) {
        self.state = new_state;
    }
}

pub fn graphics_manager(
    mut light_query: Query<(&mut Sprite, &Light)>
) {
    for (mut sprite, light) in light_query.iter_mut() {
        sprite.color = if light.state { Color::RED } else {Color::DARK_GRAY };
    }
}