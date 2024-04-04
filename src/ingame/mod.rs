use bevy::prelude::*;

pub mod crosshair;
pub mod gun;
pub mod ingame_setup;
pub mod player;
pub mod settings;
pub mod targets;
pub mod ui;

use crosshair::*;
use gun::*;
use ingame_setup::*;
use player::*;
use settings::*;
use targets::*;
use ui::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct GameSettings {
    pub sensitivity: f32,
    pub fov: f32,
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                player_setup,
                setup,
                target_setup,
                crosshair_setup,
                //setup_ui
            ),
        )
        .add_systems(
            Update,
            (
                //gun systems
                shooting_event,
                p226_firerate_timer,
                p226_animation_setup,
                p226_play_animation,
                print_hits,
                //player systems
                player_look,
                edit_mode_toggler,
                //target systems
                circle_target_controller,
                silhouette_target_controller,
                silhouette_target_hostage_controller,
                //settings system
                egui_settings,
                //png_crosshair_changer,
            ),
        )
        //plugins
        //resources
        .insert_resource(GameSettings {
            sensitivity: 0.02,
            fov: 90.0,
        })
        .insert_resource(InnerLineSettings {
            offset: 5.0,
            color: Color::WHITE,
            length: 5.0,
            thickness: 2.0,
            enable: InheritedVisibility::VISIBLE,
        })
        .init_resource::<InputState>()
        //events
        .add_event::<P226ShootingEvent>();
    }
}
