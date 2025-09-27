use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPrimaryContextPass;

pub mod crosshair;
pub mod settings;
pub mod ui;

use crosshair::*;
use settings::*;
use ui::*;

use crate::ingame::PlayableState;

#[derive(Event)]
pub struct HitmarkerEvent;

pub struct IngameUIPlugin;

impl Plugin for IngameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (crosshair_setup, ui_setup))
            .add_systems(
                EguiPrimaryContextPass,
                egui_settings.run_if(in_state(PlayableState::Menu)),
            )
            .add_systems(OnEnter(PlayableState::Menu), setting_bg)
            .add_systems(OnExit(PlayableState::Menu), despawn_bg)
            .add_systems(
                FixedUpdate,
                (
                    // ui systems
                    fps_text_updater,
                    ammo_text_updater,
                    weapon_name_text_updater,
                    velocity_text_updater,
                    target_text_updater,
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
