use bevy::{
    prelude::*,
    log::LogPlugin,
};
use bevy_mod_picking::prelude::*;

mod game;

use game::GamePlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Logic simulator".into(),
                ..default()
            }),
            ..default()
        })
        .set(LogPlugin {
            filter: "info,wgpu=error,naga=warn,bevy_mod_picking=warn".into(),
            level: bevy::log::Level::WARN
        })
    );
    app.add_plugins(DefaultPickingPlugins);

    app.add_plugins(GamePlugin);

    app.run();
}
