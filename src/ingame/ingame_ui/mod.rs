use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContextPass;

pub mod crosshair;
pub mod settings;
pub mod ui;

use crosshair::*;
use settings::*;
use ui::*;

#[derive(Event)]
pub struct HitmarkerEvent;

pub struct IngameUIPlugin;

impl Plugin for IngameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (crosshair_setup, ui_setup))
            .add_systems(EguiContextPass, egui_settings)
            .add_systems(
                FixedUpdate,
                (
                    // ui systems
                    fps_text_updater,
                    ammo_text_updater,
                    weapon_name_text_updater,
                    velocity_text_updater,
                    //target_text_updater,
                    // hitmarker systems
                    hitmarker_spawner,
                    hitmarker_controller,
                ),
            )
            // events
            .add_event::<HitmarkerEvent>()
            // resources
            .init_resource::<CrosshairLineSettings>();
    }
}
