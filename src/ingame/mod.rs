use bevy::prelude::*;

pub mod player;

use player::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    player_look,
                    run_animation,
                    keyboard_animation_control,
                    edit_mode_toggler,
                ),
            )
            .insert_resource(SensitivitySettings { sensitivity: 0.10 })
            .init_resource::<InputState>();
    }
}
