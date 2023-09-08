use bevy::prelude::*;

use super::*;

#[derive(Component, Clone)]
pub struct Chip {
    pub chip_type: ChipType,
    pub input_pins: Vec<Pin>,
    pub output_pins: Vec<Pin>,
    pub state: ChipState,
}

impl Chip {
    pub fn new(chip_type: ChipType, inputs: u8) -> Self {
        Self {
            chip_type,
            input_pins: (0..inputs).map(|_| Pin::new()).collect(),
            output_pins: (0..1).map(|_| Pin::new()).collect(),
            state: ChipState::Neutral,
        }
    }
    pub fn click(&mut self) {
        match self.chip_type {
            ChipType::Switch => {
                self.state = if self.state == ChipState::Neutral {
                    ChipState::SwitchOn
                } else {
                    ChipState::Neutral
                }
            },
            _ => (),
        }
    }
}

pub fn graphics_manager(mut chip_query: Query<(&mut Handle<Image>, &Chip)>, asset_server: Res<AssetServer>) {
    for (mut texture, chip) in chip_query.iter_mut() {
        match chip.chip_type {
            ChipType::Switch => {
                *texture = if chip.state == ChipState::SwitchOn {
                    asset_server.load("misc/Switch_On.png")
                } else {
                    asset_server.load("misc/Switch_Off.png")
                };
            }
            _ => (),
        }
    }
}

#[derive(Copy, Clone)]
pub enum ChipType {
    Switch,
    Light,
    GateAnd,
    GateOr,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ChipState {
    Neutral,
    SwitchOn,
    Lit,
}
