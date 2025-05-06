use bevy::prelude::*;

pub mod ingame_setup;
pub mod ingame_ui;
pub mod player_controller;
pub mod targets;
pub mod weapon_controller;

use ingame_setup::*;
use ingame_ui::*;
use player_controller::*;
use targets::*;
use weapon_controller::*;

#[derive(Resource)]
pub struct GameSettings {
    pub sensitivity: f32,
    pub player_speed: f32,
    pub fov: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            sensitivity: 1.0,
            player_speed: 10.0,
            fov: 90.0,
        }
    }
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, target_setup))
            .add_systems(
                Update,
                (
                    edit_mode_toggler,
                    //target systems
                    hit_detector,
                    circle_target_controller,
                    enemy_target_controller,
                    enemy_target_hostage_controller,
                ),
            )
            //events
            .add_event::<CircleTargetEvent>()
            //resources
            .init_resource::<GameSettings>()
            //states
            .init_state::<PlayableState>()
            //plugins
            .add_plugins(PlayerControllerPlugin)
            .add_plugins(IngameUIPlugin)
            .add_plugins(WeaponControllerPlugin);
    }
}
