use bevy::{prelude::*, transform::TransformSystem};

pub mod body_control;
pub mod head_control;
pub mod player;

use bevy_xpbd_3d::{parry::na::ComplexField, PhysicsSet};
use body_control::*;
use head_control::*;
use player::*;

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

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            jump: KeyCode::Space,
            run: KeyCode::ShiftLeft,
        }
    }
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

impl Default for MovementControl {
    fn default() -> Self {
        MovementControl {
            max_velocity_air: 0.6,
            max_velocity_ground: 6.0,
            max_acceleration: 10.0 * 6.0, /* max_velocity_ground */
            gravity: 15.34,
            stop_speed: 1.5,
            jump_impulse: (2.0 * 15.34 /* gravity */ * 0.85).sqrt(),
        }
    }
}

#[derive(Resource, Default)]
pub struct MovementInput {
    pub fmove: f32,
    pub smove: f32,
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup,))
            .add_systems(
                Update,
                (
                    //player systems
                    edit_mode_toggler,
                    player_move,
                    movement_input_changer,
                    player_look,
                ),
            )
            .add_systems(
                PostUpdate,
                camera_follow_player
                    .after(player_move)
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            //events
            //plugins
            //resources
            .init_resource::<KeyBindings>()
            .init_resource::<MovementControl>()
            .init_resource::<MovementInput>();
    }
}
