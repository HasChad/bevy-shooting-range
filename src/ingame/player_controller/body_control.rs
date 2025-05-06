use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ingame::GameSettings;

use super::{
    player::{GroundChecker, Player},
    KeyBindings,
};

pub fn player_position_reset(
    key_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    if key_input.just_pressed(KeyCode::KeyX) {
        player_transform.translation = Vec3::new(0.0, 0.5, 0.0);
    }
}

pub fn player_move(
    time: Res<Time>,
    settings: Res<GameSettings>,
    key_bindings: Res<KeyBindings>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    for (mut linear_velocity, player_transform) in player_query.iter_mut() {
        let (yaw_player, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);

        // ! player movement input
        let mut fmove = 0.0;
        let mut smove = 0.0;

        for key in input.get_pressed() {
            let key = *key;

            if key == key_bindings.move_forward {
                fmove = 1.0;
            } else if key == key_bindings.move_backward {
                fmove = -1.0
            }

            if key == key_bindings.move_left {
                smove = 1.0
            } else if key == key_bindings.move_right {
                smove = -1.0
            }

            if key == key_bindings.jump {
                linear_velocity.y = 3.0
            }
        }

        // ! player looking direction
        let forward = Vec3::new(-yaw_player.sin(), 0.0, -yaw_player.cos()).normalize();
        let right = Vec3::new(-yaw_player.cos(), 0.0, yaw_player.sin()).normalize();

        // ! wishvel
        let wish_vel = Vec3::new(
            forward.x * fmove + right.x * smove,
            0.0,
            forward.z * fmove + right.z * smove,
        )
        .normalize_or_zero();

        let mut wish_speed = wish_vel.length() * settings.player_speed;

        if wish_speed > 5.0 {
            wish_speed = 5.0
        }

        let current_speed = linear_velocity.dot(wish_vel);

        let add_speed = wish_speed - current_speed;

        if add_speed <= 0.0 {
            return;
        } else {
            let accel_speed = time.delta_secs() * settings.player_speed;

            **linear_velocity += accel_speed * wish_vel;
        }
    }
}

pub fn ground_check(
    collisions: Collisions,
    gc_entity: Single<Entity, With<GroundChecker>>,
    mut player_query: Query<&mut LinearVelocity, With<Player>>,
) {
    for contacts in collisions.iter() {
        for mut player_lin_vel in player_query.iter_mut() {
            if contacts.collider1 == *gc_entity || contacts.collider2 == *gc_entity {
                if player_lin_vel.length() < 1.0 {
                    player_lin_vel.x = 0.0;
                    player_lin_vel.y = 0.0;
                    player_lin_vel.z = 0.0;
                }

                if player_lin_vel.x > 0.0 {
                    player_lin_vel.x -= 0.2;
                } else if player_lin_vel.x < 0.0 {
                    player_lin_vel.x += 0.2;
                }

                if player_lin_vel.z > 0.0 {
                    player_lin_vel.z -= 0.2;
                } else if player_lin_vel.z < 0.0 {
                    player_lin_vel.z += 0.2;
                }
            }
        }
    }
}
