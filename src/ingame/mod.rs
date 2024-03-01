use bevy::prelude::*;

pub mod fps_counter;
pub mod gun;
pub mod ingame_setup;
pub mod player;
pub mod targets;

use fps_counter::*;
use gun::*;
use ingame_setup::*;
use player::*;
use targets::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (player_setup, setup, first_target_setup, setup_fps_counter),
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
                //fps counter system
                fps_text_update_system,
            ),
        )
        //resources
        .insert_resource(SensitivitySettings { sensitivity: 0.02 })
        .init_resource::<InputState>()
        //events
        .add_event::<P226ShootingEvent>();
    }
}
