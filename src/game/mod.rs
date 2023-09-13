use std::sync::{Arc, Mutex};

use bevy::prelude::*;

mod components;
mod sim_manager;
mod events;
mod systems;

use components::*;
use sim_manager::*;
use events::*;
use systems::*;
use uuid::Uuid;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Registering events
        app.add_event::<ChipPlacedEvent>();
        app.add_event::<ChipClicked>();

        app.add_systems(Startup, setup_engine);
        app.add_systems(PostStartup, add_logic_gates);
        app.add_systems(PostUpdate, add_logic_gate_to_world);

        app.add_systems(Update, chip_graphics_manager);
    }
}
fn setup_engine(mut commands: Commands) {
    println!("Setting up simulation.");
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SimManager::new());
}

fn add_logic_gates(
    mut manager_query: Query<&mut SimManager>,
    mut logic_gate_place_event_tx: EventWriter<ChipPlacedEvent>
) {
    let mut manager = manager_query.single_mut();
    // Adding chips to the simulation.
    let switch1 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::Switch,Vec3::new(-100.0, 50.0, 0.0));
    let switch2 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::Switch,Vec3::new(-100.0, -50.0, 0.0));
    let gate = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::GateOr, Vec3::new(0.0, 0.0, 0.0));
    let light = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::Light, Vec3::new(100.0, 0.0, 0.0));

    
    switch1.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(gate.clone()), 0))));
    switch2.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(gate.clone()), 1))));
    gate.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(light.clone()), 0))));
    // logic_gate_place_event_tx.send(ChipPlacedEvent::new(Chip::new(ChipType::GateAnd, 2), Vec3::new(0.0, 0.0, 0.0)));
    // logic_gate_place_event_tx.send(ChipPlacedEvent::new(Chip::new(ChipType::Light, 1), Vec3::new(100.0, 0.0, 0.0)));
}
fn place_chip(manager: &mut SimManager, event_tx: &mut EventWriter<ChipPlacedEvent>, chip_type: ChipType, translation: Vec3) -> Arc<Mutex<Chip>> {
    let uuid = Uuid::new_v4();
    let chip = Chip::new(chip_type);
    println!("Inserting new {:?}, with UUID: '{}'", chip.chip_type, uuid);
    manager.chip_map.insert(uuid, Arc::new(Mutex::new(chip.clone())));
    
    event_tx.send(ChipPlacedEvent::new(chip.chip_type, translation, uuid));
    return manager.chip_map.get(&uuid).unwrap().clone();
}

// Chips are stored in a hashmap with UUIDs
// For each chip, x amount of output pins are made
// These are also stored in a hashmap.
// The chip can access these pins with an Arc.
// The pins have a vector of Arcs to the chips they are connected to.