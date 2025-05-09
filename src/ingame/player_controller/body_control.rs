use avian3d::prelude::*;
use bevy::{math::VectorSpace, prelude::*};

use crate::ingame::GameSettings;

use super::{
    player::{GroundChecker, Player},
    KeyBindings,
};

#[derive(Resource, Default)]
pub struct MovementInput {
    pub fmove: f32,
    pub smove: f32,
}

pub fn player_position_reset(
    key_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    if key_input.just_pressed(KeyCode::KeyX) {
        player_transform.translation = Vec3::new(0.0, 0.5, 0.0);
    }
}

pub fn movement_input_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut movement: ResMut<MovementInput>,
    key_bindings: Res<KeyBindings>,
) {
    if input.pressed(key_bindings.move_forward) && input.pressed(key_bindings.move_backward) {
        movement.fmove = 0.0
    } else if input.pressed(key_bindings.move_forward) {
        movement.fmove = 1.0;
    } else if input.pressed(key_bindings.move_backward) {
        movement.fmove = -1.0
    } else {
        movement.fmove = 0.0
    }

    if input.pressed(key_bindings.move_left) && input.pressed(key_bindings.move_right) {
        movement.smove = 0.0
    } else if input.pressed(key_bindings.move_left) {
        movement.smove = 1.0
    } else if input.pressed(key_bindings.move_right) {
        movement.smove = -1.0
    } else {
        movement.smove = 0.0
    }

    /*
    if key == key_bindings.jump && player_promp.on_ground {
        lin_vel.y = 5.0
    }
    */
}

pub fn player_move(
    settings: Res<GameSettings>,
    movement: Res<MovementInput>,
    mut player_query: Query<(&mut LinearVelocity, &mut Transform, &Player)>,
) {
    for (mut lin_vel, player_transform, player_promp) in player_query.iter_mut() {
        let (yaw_player, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);

        // ! player looking direction
        let forward = Vec3::new(-yaw_player.sin(), 0.0, -yaw_player.cos()).normalize();
        let right = Vec3::new(-yaw_player.cos(), 0.0, yaw_player.sin()).normalize();

        // ! wishvel
        let wish_vel = Vec3::new(
            forward.x * movement.fmove + right.x * movement.smove,
            0.0,
            forward.z * movement.fmove + right.z * movement.smove,
        )
        .normalize_or_zero();

        let mut wish_speed = wish_vel.length() * settings.player_speed;

        if wish_speed > 0.5 {
            wish_speed = 0.5
        }

        let current_speed = lin_vel.dot(wish_vel);

        let add_speed = wish_speed - current_speed;

        // info!("wish speed" = wish_speed);
        // info!("current speed" = current_speed);
        // info!("add speed" = add_speed);

        if player_promp.on_ground {
            let mut friction_direction = Vec3::new(-lin_vel.x, 0.0, -lin_vel.z).normalize_or_zero();

            if lin_vel.length() <= 0.2 {
                lin_vel.x = 0.0;
                lin_vel.z = 0.0;
            }

            if lin_vel.length() == 0.0 {
                friction_direction = Vec3::ZERO
            }

            info!("len" = lin_vel.length());
            info!("fri len" = friction_direction.length());

            if friction_direction.length() > 0.0 {
                **lin_vel += friction_direction * 0.2
            }
        }

        if add_speed <= 0.0 {
            return;
        } else {
            let accel_speed = settings.player_speed;

            **lin_vel += add_speed * wish_vel;
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
