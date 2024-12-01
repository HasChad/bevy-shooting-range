use avian3d::prelude::*;
use bevy::prelude::*;

use super::{player::Player, KeyBindings};

pub fn player_position_reset(
    key_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if key_input.just_pressed(KeyCode::KeyX) {
        let mut player_pos = player_query.single_mut();
        player_pos.translation = Vec3::new(0.0, 0.5, 0.0);
    }
}

pub fn player_move(
    key_bindings: Res<KeyBindings>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    let mut fmove = 0.0;
    let mut smove = 0.0;

    for key in input.get_pressed() {
        let key = *key;

        if key == key_bindings.move_forward {
            fmove = 5.0;
        } else if key == key_bindings.move_backward {
            fmove = -5.0
        }

        if key == key_bindings.move_left {
            smove = 5.0
        } else if key == key_bindings.move_right {
            smove = -5.0
        }
    }

    for (mut linear_velocity, player_transform) in player_query.iter_mut() {
        let (yaw_player, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);

        // ! player looking direction
        let forward = Vec3::new(-yaw_player.sin(), 0.0, -yaw_player.cos()).normalize();
        let right = Vec3::new(-yaw_player.cos(), 0.0, yaw_player.sin()).normalize();

        // ! wishvel
        let wishvel = Vec3::new(
            forward.x * fmove + right.x * smove,
            0.0,
            forward.z * fmove + right.z * smove,
        );

        let wishdir = wishvel.normalize_or_zero();

        linear_velocity.x = wishdir.x * 5.;
        linear_velocity.z = wishdir.z * 5.;
    }
}
