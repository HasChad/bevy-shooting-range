use bevy::prelude::*;

pub mod player;
pub mod player_look;
pub mod player_movement;

use bevy_xpbd_3d::parry::na::ComplexField;
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

#[derive(Resource)]
pub struct MovementControl {
    pub max_velocity_air: f32,
    pub max_velocity_ground: f32,
    pub max_acceleration: f32,
    pub gravity: f32,
    pub stop_speed: f32,
    pub jump_impulse: f32,
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup,))
            .add_systems(
                Update,
                (
                    //player systems
                    player_look,
                    player_move,
                    edit_mode_toggler,
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
                max_velocity_air: 0.6,
                max_velocity_ground: 6.0,
                max_acceleration: 10.0 * 6.0, /* max_velocity_ground */
                gravity: 15.34,
                stop_speed: 1.5,
                jump_impulse: (2.0 * 15.34 /* gravity */ * 0.85).sqrt(),
            });

        //events
    }
}
