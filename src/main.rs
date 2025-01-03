// #![windows_subsystem = "windows"]

use avian3d::prelude::*;
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
use bevy_kira_audio::prelude::*;
use color_eyre::eyre::Result;

pub mod ingame;

use ingame::*;

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
        //plugins
        .add_plugins(AudioPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(PhysicsPlugins::default())
        //4
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(WorldInspectorPlugin::default())
        //mod plugins
        .add_plugins(InGamePlugin)
        .run();

    Ok(())
}
