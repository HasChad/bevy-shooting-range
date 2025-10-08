#![windows_subsystem = "windows"]

use avian3d::prelude::*;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub mod ingame;

use ingame::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Shooting Range".into(),

                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        //plugins
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_gizmo_config(
            PhysicsGizmos {
                contact_point_color: Some(Color::BLACK),
                contact_normal_color: Some(Color::BLACK),
                ..default()
            },
            GizmoConfig::default(),
        )
        .add_plugins(EguiPlugin::default())
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)),
        )
        //mod plugins
        .add_plugins(InGamePlugin)
        .run();
}
