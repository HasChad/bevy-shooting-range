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
                    //gun systems
                    shooting_event,
                    p226_firerate_timer,
                    p226_animation_setup,
                    p226_play_animation,
                    print_hits,
                    //player systems
                    player_look,
                    edit_mode_toggler,
                ),
            )
            //resources
            .insert_resource(SensitivitySettings { sensitivity: 0.10 })
            .init_resource::<InputState>()
            //events
            .add_event::<P226ShootingEvent>();
    }
}
