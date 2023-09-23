use std::sync::Arc;

use bevy::{prelude::*, core_pipeline::core_2d::graph::input};

use crate::game::*;

#[derive(Component, Debug, Clone)]
pub struct Chip {
    pub chip_type: ChipType,
    pub state: ChipState,
    pub input_pins: Vec<bool>,
    // pub output_pin: Box<Pin>,
    pub output_pins: Vec<Arc<Mutex<Pin>>>,
}

impl Chip {
    pub fn new(chip_type: ChipType) -> Self {
        let num_inputs = match chip_type {
            ChipType::GateAnd |
            ChipType::GateOr => 2,
            ChipType::GateNot => 1,
            ChipType::Switch => 0,
            ChipType::Light => 1
        };
        Self {
            chip_type,
            // input_pins: (0..inputs).map(|_| Pin::new()).collect(),
            // output_pins: (0..1).map(|_| Pin::new()).collect(),
            state: ChipState::Neutral,
            input_pins: vec![false; num_inputs],
            // output_pin: Box::new(Pin::new(None)),
            output_pins: Vec::new(), // Will be filled with Arcs later.
        }
    }
    pub fn receive_input(&mut self, pin_index: u8, new_state: bool) {
        println!("{:?} is receiving input", self.chip_type);
        self.input_pins[pin_index as usize] = new_state;
        self.think_really_hard();
        // let _ = std::mem::replace(&mut self.input_pins[pin_index as usize], new_state);

    }
    pub fn think_really_hard(&mut self) {
        println!("{:?} is thinking really hard", self.chip_type);
        let mut output = None;
        match self.chip_type {
            ChipType::GateAnd => {
                output = Some(!self.input_pins.contains(&false));
            }
            ChipType::GateOr => {
                output = Some(self.input_pins.contains(&true));
            }
            ChipType::GateNot => {
                output = Some(match self.input_pins.get(0) {
                    Some(input_value) => !input_value, 
                    None => false
                });
            }
            ChipType::Light => {
                self.state = if self.input_pins.contains(&true) {
                    ChipState::Lit
                } else {
                    ChipState::Neutral
                };
            }
            _ => ()
        };
        if let Some(out) = output {
            for output_pin in self.output_pins.iter() {
                output_pin.lock().unwrap().update_state(out);
            }
        }
    }

    pub fn click(&mut self) {
        match self.chip_type {
            ChipType::Switch => {
                self.state = if self.state == ChipState::Neutral {
                    ChipState::SwitchOn
                } else {
                    ChipState::Neutral
                };
                // self.output_pins.iter_mut()
                for output_pin in self.output_pins.iter() {
                    output_pin.lock().unwrap().update_state(self.state == ChipState::SwitchOn);
                }
            },
            _ => (),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ChipType {
    Switch,
    Light,
    GateAnd,
    GateOr,
    GateNot,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ChipState {
    Neutral,
    SwitchOn,
    Lit,
}
