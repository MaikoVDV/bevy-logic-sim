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
    let switch1 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::Switch,Vec3::new(-200.0, 50.0, 0.0));
    let switch2 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::Switch,Vec3::new(-200.0, -50.0, 0.0));

    let or_gate1 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::GateOr, Vec3::new(-50.0, 50.0, 0.0));
    let and_gate1 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::GateAnd, Vec3::new(-100.0, -50.0, 0.0));
    let not_gate1 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::GateNot, Vec3::new(-50.0, -50.0, 0.0));

    let and_gate2 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::GateAnd, Vec3::new(0.0, 0.0, 0.0));

    let light1 = place_chip(&mut manager, &mut logic_gate_place_event_tx, ChipType::Light, Vec3::new(100.0, 0.0, 0.0));

    
    // connecting switches to OR
    switch1.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(or_gate1.clone()), 0))));
    switch2.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(or_gate1.clone()), 1))));
    // connecting switches to AND1
    switch1.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(and_gate1.clone()), 0))));
    switch2.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(and_gate1.clone()), 1))));
    // connecting AND1 to NOT
    and_gate1.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(not_gate1.clone()), 0))));
    // connecting OR to AND2
    or_gate1.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(and_gate2.clone()), 0))));
    // connecting NOT to AND2
    not_gate1.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(and_gate2.clone()), 1))));
    // connecting AND2 to LIGHT
    and_gate2.lock().unwrap().output_pins.push(Arc::new(Mutex::new(Pin::new(Some(light1.clone()), 0))));


    for (_, chip) in manager.chip_map.iter() {
        chip.lock().unwrap().think_really_hard();
    }
}
fn place_chip(manager: &mut SimManager, event_tx: &mut EventWriter<ChipPlacedEvent>, chip_type: ChipType, translation: Vec3) -> Arc<Mutex<Chip>> {
    let uuid = Uuid::new_v4();
    println!("Inserting new {:?}, with UUID: '{}'", chip_type, uuid);

    let chip_ref = Arc::new(Mutex::new(Chip::new(chip_type)));
    manager.chip_map.insert(uuid, chip_ref.clone());
    
    event_tx.send(ChipPlacedEvent::new(chip_ref.clone(), chip_type, translation, uuid));
    return chip_ref.clone();
}

// Chips are stored in a hashmap with UUIDs
// For each chip, x amount of output pins are made
// These are also stored in a hashmap.
// The chip can access these pins with an Arc.
// The pins have a vector of Arcs to the chips they are connected to.