use avian3d::prelude::*;
use bevy::{prelude::*, transform::TransformSystem};

pub mod body_control;
pub mod head_control;
pub mod player;

use body_control::*;
use head_control::*;
use player::*;

use super::PlayableState;

/// Key configuration
#[derive(Resource)]
pub struct KeyBindings {
    pub fire: MouseButton,
    pub scope: MouseButton,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub run: KeyCode,
    pub reload: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            fire: MouseButton::Left,
            scope: MouseButton::Right,
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            jump: KeyCode::Space,
            run: KeyCode::ShiftLeft,
            reload: KeyCode::KeyR,
        }
    }
}

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup,))
            .add_systems(
                Update,
                (
                    //player systems
                    (player_move, player_look, player_position_reset)
                        .run_if(in_state(PlayableState::Action)),
                ),
            )
            .add_systems(
                PostUpdate,
                camera_follow_player
                    .after(player_move)
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            //resources
            .init_resource::<KeyBindings>();
    }
}
