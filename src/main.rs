use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::prelude::*;
use color_eyre::eyre::Result;

pub mod ingame;

use ingame::InGamePlugin;

fn main() -> Result<()> {
    color_eyre::install()?;
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Shooting Range".into(),
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        //plugins
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        //mod plugins
        .add_plugins(InGamePlugin)
        .run();

    Ok(())
}
