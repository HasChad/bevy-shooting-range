use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active,
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
        //plugins
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::F35)),
        )
        //mod plugins
        .add_plugins(InGamePlugin)
        //systems
        .add_systems(Update, fps_text_update_system)
        .add_systems(Startup, setup_fps_counter)
        .run();

    Ok(())
}
