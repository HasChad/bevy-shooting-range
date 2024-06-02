use bevy::prelude::*;

pub mod crosshair;
pub mod gun;
pub mod ingame_setup;
pub mod player_controller;
pub mod settings;
pub mod targets;
pub mod ui;

use crosshair::*;
use gun::*;
use ingame_setup::*;
use player_controller::PlayerControllerPlugin;
use settings::*;
use targets::*;
use ui::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct GameSettings {
    pub sensitivity: f32,
    pub player_speed: f32,
    pub fov: f32,
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, target_setup, crosshair_setup, setup_ui))
            .add_systems(
                Update,
                (
                    //gun systems
                    shooting_event,
                    firerate_timer,
                    weapon_animation_setup,
                    weapon_play_animation,
                    print_hits,
                    reload_timer,
                    //target systems
                    circle_target_controller,
                    silhouette_target_controller,
                    silhouette_target_hostage_controller,
                    //settings system
                    egui_settings,
                    //ui systems
                    velocity_text_updater,
                    weapon_name_text_updater,
                    ammo_text_updater,
                ),
            )
            //plugins
            .add_plugins(PlayerControllerPlugin)
            //resources
            .insert_resource(GameSettings {
                sensitivity: 1.0,
                player_speed: 5.0,
                fov: 90.0,
            })
            .insert_resource(InnerLineSettings {
                offset: 5.0,
                color: Color::WHITE,
                length: 5.0,
                thickness: 2.0,
                enable: InheritedVisibility::VISIBLE,
            })
            //events
            .add_event::<WeaponShootingEvent>()
            .add_event::<WeaponReloadingEvent>();
    }
}
