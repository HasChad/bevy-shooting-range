use bevy::{prelude::*, window::PrimaryWindow};
use bevy_xpbd_3d::prelude::*;

use crate::ingame::player_controller::player::Player;
use crate::ingame::player_controller::KeyBindings;
use crate::ingame::GameSettings;

use super::MovementControl;

#[derive(Component)]
pub struct VelocityText;

pub fn player_move(
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut movement_control: ResMut<MovementControl>,
    input: Res<ButtonInput<KeyCode>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    key_bindings: Res<KeyBindings>,
    mut query_player: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut linear_velocity, player_transform) in query_player.iter_mut() {
            let (yaw_player, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);

            let mut fmove = 0.0;
            let mut smove = 0.0;

            /*
            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {

                }
            }
            */

            //get input
            for key in input.get_pressed() {
                let key = *key;

                if key == key_bindings.move_forward {
                    fmove = 5.0
                } else if key == key_bindings.move_backward {
                    fmove = -5.0
                }

                if key == key_bindings.move_left {
                    smove = 5.0
                } else if key == key_bindings.move_right {
                    smove = -5.0
                }

                if input.just_pressed(key_bindings.jump) {
                    linear_velocity.y = 5.0;
                }
            }

            for key in input.get_just_released() {
                let key = *key;

                if key == key_bindings.move_forward || key == key_bindings.move_backward {
                    fmove = 0.0
                }

                if key == key_bindings.move_left || key == key_bindings.move_right {
                    smove = 0.0
                }
            }

            // ! player looking direction
            let forward = vector_normalize(Vec3::new(-yaw_player.sin(), 0.0, -yaw_player.cos()));
            let right = vector_normalize(Vec3::new(-yaw_player.cos(), 0.0, yaw_player.sin()));

            // ! wishvel
            let wishvel = Vec3::new(
                forward.x * fmove + right.x * smove,
                0.0,
                forward.z * fmove + right.z * smove,
            );

            let wishdir = wishvel;
            let wishspeed = vector_normalize(wishdir);

            linear_velocity.x = wishspeed.x * 5.0;
            linear_velocity.z = wishspeed.z * 5.0;
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

fn process_movement(time: Res<Time>, wishdir: Vec3) {
    let velocity = update_velocity_ground(time, wishdir);
}

fn update_velocity_ground(time: Res<Time>, wishdir: Vec3) {
    let mut speed = wishdir.length();
}

fn update_velocity_air(wishdir: Vec3) {}

fn accelerate(wishdir: Vec3, max_speed: f32) {}

fn vector_normalize(mut v: Vec3) -> Vec3 {
    let mut length: f32;

    length = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    length = length.sqrt(); // FIXME

    if length > 0.0 {
        let ilength = 1.0 / length;
        v[0] *= ilength;
        v[1] *= ilength;
        v[2] *= ilength;
    }

    v
}
