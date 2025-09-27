#![windows_subsystem = "windows"]

use avian3d::prelude::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::WindowMode};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_kira_audio::prelude::*;

pub mod ingame;

use ingame::*;

fn main() {
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
        .add_plugins(AudioPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        //.add_plugins(WorldInspectorPlugin::default())
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        //mod plugins
        .add_plugins(InGamePlugin)
        .run();
}
