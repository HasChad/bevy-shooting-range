use bevy::window::CursorGrabMode;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_3d::prelude::*;

use crate::ingame::player_controller::player::Player;
use crate::ingame::player_controller::KeyBindings;

use super::MovementInput;

pub fn movement_input_changer(
    mut movement_input: ResMut<MovementInput>,
    key_bindings: Res<KeyBindings>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // ! get fmove and smove
    for key in input.get_pressed() {
        let key = *key;

        if key == key_bindings.move_forward {
            movement_input.fmove = 5.0;
        } else if key == key_bindings.move_backward {
            movement_input.fmove = -5.0
        }

        if key == key_bindings.move_left {
            movement_input.smove = 5.0
        } else if key == key_bindings.move_right {
            movement_input.smove = -5.0
        }

        /*
        if input.just_pressed(key_bindings.jump) {
            linear_velocity.y = 5.0;
        }
        */
    }

    for key in input.get_just_released() {
        let key = *key;

        if key == key_bindings.move_forward || key == key_bindings.move_backward {
            movement_input.fmove = 0.0
        }

        if key == key_bindings.move_left || key == key_bindings.move_right {
            movement_input.smove = 0.0
        }
    }
}

pub fn player_move(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    movement_input: Res<MovementInput>,
    mut query_player: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut linear_velocity, player_transform) in query_player.iter_mut() {
            let (yaw_player, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    // ! player looking direction
                    let forward =
                        vector_normalize(Vec3::new(-yaw_player.sin(), 0.0, -yaw_player.cos()));
                    let right =
                        vector_normalize(Vec3::new(-yaw_player.cos(), 0.0, yaw_player.sin()));

                    // ! wishvel
                    let wishvel = Vec3::new(
                        forward.x * movement_input.fmove + right.x * movement_input.smove,
                        0.0,
                        forward.z * movement_input.fmove + right.z * movement_input.smove,
                    );

                    let wishdir = vector_normalize(wishvel);

                    linear_velocity.x = wishdir.x * 5.;
                    linear_velocity.z = wishdir.z * 5.;

                    // ! changed from linear_velocity to player_transform.translation
                    //player_transform.translation.x += wishdir.x * 0.03;
                    //player_transform.translation.z += wishdir.z * 0.03;
                }
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

fn vector_normalize(mut v: Vec3) -> Vec3 {
    let mut length: f32;

    length = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    length = length.sqrt();

    if length > 0.0 {
        let ilength = 1.0 / length;
        v[0] *= ilength;
        v[1] *= ilength;
        v[2] *= ilength;
    }

    v
}
