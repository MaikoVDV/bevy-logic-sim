use bevy::prelude::*;
use crate::game::*;

pub fn chip_graphics_manager(
    mut chip_query: Query<(&mut Handle<Image>, &ChipRef)>,
    mut manager_query: Query<&mut SimManager>,
    asset_server: Res<AssetServer>
) {
    let mut manager = manager_query.single_mut();
    for (mut texture, chip_ref) in chip_query.iter_mut() {
        match manager.chip_map.get_mut(&chip_ref.0) {
            Some(chip_ref) => {
                let chip = chip_ref.lock().unwrap();
                match chip.chip_type {
                    ChipType::Switch => {
                        *texture = if chip.state == ChipState::SwitchOn {
                            asset_server.load("misc/Switch_On.png")
                        } else {
                            asset_server.load("misc/Switch_Off.png")
                        };
                    }
                    ChipType::Light => {
                        *texture = if chip.state == ChipState::Lit {
                            asset_server.load("misc/Light_On.png")
                        } else {
                            asset_server.load("misc/Light_Off.png")
                        };
                    }
                    _ => (),
                }
                
            }
            None => {

            }
        }
        
    }
}