use bevy::prelude::*;

pub mod gun;
pub mod ingame_setup;
pub mod player;

use gun::*;
use ingame_setup::*;
use player::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup, setup))
            .add_systems(
                Update,
                (
                    player_look,
                    run_animation,
                    keyboard_animation_control,
                    edit_mode_toggler,
                    print_hits,
                    //spawn_bullet,
                ),
            )
            .insert_resource(SensitivitySettings { sensitivity: 0.10 })
            .init_resource::<InputState>()
            .add_event::<P226ShootingEvent>();
    }
}
