use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
    window::WindowMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::prelude::*;
use color_eyre::eyre::Result;

pub mod fps_counter;
pub mod ingame;

use fps_counter::*;
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
                .set(RenderPlugin {
                    render_creation: {
                        bevy::render::settings::RenderCreation::Automatic(WgpuSettings {
                            backends: Some(Backends::VULKAN),
                            ..default()
                        })
                    },
                    ..default()
                })
                .build(),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        //plugins
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        //mod plugins
        .add_plugins(InGamePlugin)
        //systems
        .add_systems(Update, fps_text_update_system)
        .add_systems(Startup, setup_fps_counter)
        .run();

    Ok(())
}
