use bevy::prelude::*;

mod components;
mod sim_manager;
mod events;

use components::*;
use sim_manager::*;
use events::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Registering events
        app.add_event::<ChipPlacedEvent>();

        app.add_systems(Startup, setup_engine);
        app.add_systems(Update, add_logic_gate_to_world);

        app.add_systems(Update, graphics_manager);
        // app.add_systems(Update, light::graphics_manager);
    }
}
fn setup_engine(mut commands: Commands, asset_server: Res<AssetServer>, mut logic_gate_place_event_tx: EventWriter<ChipPlacedEvent>) {
    println!("Setting up game.");
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SimManager::new(logic_gate_place_event_tx));
}