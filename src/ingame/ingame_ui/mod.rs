use bevy::prelude::*;

pub mod crosshair;
pub mod settings;
pub mod ui;

use crosshair::*;
use settings::*;
use ui::*;

pub struct IngameUIPlugin;

impl Plugin for IngameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (crosshair_setup, ui_setup))
            .add_systems(Update, egui_settings)
            .add_systems(
                FixedUpdate,
                (
                    //ui systems
                    ammo_text_updater,
                    target_text_updater,
                    weapon_name_text_updater,
                    velocity_text_updater,
                    fps_text_updater,
                    //hitmarker systems
                    hitmarker_spawner,
                    hitmarker_controller,
                ),
            )
            //resources
            .init_resource::<CrosshairLineSettings>();
    }
}
