use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use super::components::*;
use super::events::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct SimManager {

}

impl SimManager {
    pub fn new(mut place_gate_writer: EventWriter<ChipPlacedEvent>) {
        let hashmap: HashMap<u32, &Pin>;

        // spawning an AND gate
        let mut inputs = Vec::new();
        inputs.push(Pin { state: false});
        inputs.push(Pin { state: false});

        place_gate_writer.send(ChipPlacedEvent::new(Chip::new(ChipType::Switch, 0), Vec3::new(-100.0, 0.0, 0.0)));
        place_gate_writer.send(ChipPlacedEvent::new(Chip::new(ChipType::GateAnd, 2), Vec3::new(0.0, 0.0, 0.0)));
        place_gate_writer.send(ChipPlacedEvent::new(Chip::new(ChipType::Light, 1), Vec3::new(100.0, 0.0, 0.0)));
        // commands.spawn((
        //     LogicGate::new(inputs), 
        //     SpriteBundle {
        //         texture: asset_server.load("logic gates/AND_gate.png"),
        //         transform: Transform {
        //             translation: Vec3::ZERO,
        //             ..default()
        //         },
        //         ..default()
        //     },
        //     On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //         transform.translation.x += drag.delta.x; // Make the square follow the mouse
        //         transform.translation.y -= drag.delta.y;
        //     }),
        // ));
        // // spawning a switch
        // commands.spawn((
        //     ManualSwitch::new(None), 
        //     SpriteBundle {
        //         texture: asset_server.load("logic gates/AND_gate.png"),
        //         transform: Transform {
        //             translation: Vec3::new(-100.0, 0.0, 0.0),
        //             ..default()
        //         },
        //         ..default()
        //     },
        //     On::<Pointer<Click>>::target_component_mut::<ManualSwitch>(|_click, switch| {
        //         switch.toggle();
        //     }),
        //     On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //         transform.translation.x += drag.delta.x; // Make the square follow the mouse
        //         transform.translation.y -= drag.delta.y;
        //     }),
        // ));
    
        // // spawning a light for debugging
        // commands.spawn((
        //     Light::new(), 
        //     SpriteBundle {
        //         texture: asset_server.load("logic gates/AND_gate.png"),
        //         transform: Transform {
        //             translation: Vec3::new(100.0, 0.0, 0.0),
        //             ..default()
        //         },
        //         ..default()
        //     },
        // ));
    }
}

pub fn add_logic_gate_to_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut logic_gate_place_event_rx: EventReader<ChipPlacedEvent>
) {
    for place_event in logic_gate_place_event_rx.iter() {
        // spawning a light for debugging
        commands.spawn((
            place_event.chip.clone(), 
            SpriteBundle {
                texture: asset_server.load(match place_event.chip.chip_type {
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
            On::<Pointer<Click>>::target_component_mut::<Chip>(|_click, chip| {
                chip.click();
            }),
        )
    );
    }
}