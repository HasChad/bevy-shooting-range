use bevy::prelude::*;

pub mod player;
pub mod player_look;
pub mod player_movement;

use player::*;
use player_look::*;
use player_movement::*;

/// Key configuration
#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub run: KeyCode,
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup, setup_velocity_counter))
            .add_systems(
                Update,
                (
                    //player systems
                    player_look,
                    player_move,
                    edit_mode_toggler,
                    velocity_update_system,
                ),
            )
            //plugins
            //resources
            .init_resource::<InputState>()
            .insert_resource(KeyBindings {
                move_forward: KeyCode::KeyW,
                move_backward: KeyCode::KeyS,
                move_left: KeyCode::KeyA,
                move_right: KeyCode::KeyD,
                jump: KeyCode::Space,
                run: KeyCode::ShiftLeft,
            })
            .insert_resource(MovementControl {
                fmove: 0.0,
                smove: 0.0,
            });

        //events
    }
}
