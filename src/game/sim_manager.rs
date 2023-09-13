use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use super::components::*;
use super::events::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Component)]
pub struct SimManager {
    pub chip_map: HashMap<Uuid, Arc<Mutex<Chip>>>
}

impl SimManager {
    pub fn new() -> Self {
        Self {
            chip_map: HashMap::new()
        }
    }
}


pub fn add_logic_gate_to_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut logic_gate_place_event_rx: EventReader<ChipPlacedEvent>,
) {
    for place_event in logic_gate_place_event_rx.iter() {
        commands.spawn((
            ChipRef(place_event.uuid),
            SpriteBundle {
                texture: asset_server.load(match place_event.chip_type {
                    ChipType::GateAnd => "logic gates/AND_gate.png",
                    ChipType::Switch => "misc/Switch_Off.png",
                    ChipType::Light => "misc/Light_Off.png",
                    _ => "logic gates/AND_gate.png"
                }),
                transform: Transform {
                    translation: place_event.translation,
                    ..default()
                },
                ..default()
            },
            On::<Pointer<DragStart>>::target_insert(Pickable::IGNORE), // Disable picking
            On::<Pointer<DragEnd>>::target_insert(Pickable::default()), // Re-enable picking
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
            }),
            On::<Pointer<Click>>::run(|
                click_event: Listener<Pointer<Click>>,
                chip_query: Query<&ChipRef>,
                mut manager_query: Query<&mut SimManager>
            |{
                let chip_ref = chip_query.get(click_event.target).unwrap();
                let mut sim_manager = manager_query.single_mut();
                let chip = sim_manager.chip_map.get_mut(&chip_ref.0).unwrap();
                chip.lock().unwrap().click();
            })
        ));
    }
}