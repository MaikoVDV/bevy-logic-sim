// use uuid::Uuid;
use std::sync::{Arc, Mutex};

use super::Chip;

#[derive(Debug, Clone)]
pub struct Pin {
    pub state: bool,
    // Has value if this pin is the input for a chip.
    // pub input_chip: Option<ChipConnection>
    pub input_chip_conn: ChipConnection
}
impl Pin {
    pub fn new(input_chip: Option<Arc<Mutex<Chip>>>, input_index: u8) -> Self {
        Self {
            state: false,
            input_chip_conn: ChipConnection(input_chip, input_index)
        }
    }

    /// Change this pin's state.
    /// Called by other pins (e.g. like output pins from chips)
    pub fn update_state(&mut self, new_state: bool) {
        self.state = new_state;

        if let Some(chip) = self.input_chip_conn.0.clone() {
            chip.lock().unwrap().receive_input(self.input_chip_conn.1, new_state);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChipConnection(Option<Arc<Mutex<Chip>>>, u8);