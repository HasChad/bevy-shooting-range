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
    mut player_query: Query<(&mut LinearVelocity, &mut Transform, &Player)>,
) {
    for (mut linear_velocity, player_transform, player_promp) in player_query.iter_mut() {
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

            if key == key_bindings.jump && player_promp.on_ground {
                linear_velocity.y = 5.0
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

        let mut add_speed = wish_speed - current_speed;

        println!("is grounded {:?}", player_promp.on_ground);

        if player_promp.on_ground {
            add_speed -= 5.0
        }

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
    mut player_promp: Single<&mut Player>,
) {
    for _ in collisions.entities_colliding_with(*gc_entity) {
        player_promp.on_ground = true;
        return;
    }

    player_promp.on_ground = false;
}
