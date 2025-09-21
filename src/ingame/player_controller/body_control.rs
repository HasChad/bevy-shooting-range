use avian3d::prelude::*;
use bevy::prelude::*;

use crate::ingame::player::Head;
use crate::ingame::GameSettings;

use super::{player::Player, KeyBindings};

const PLAYER_SPEED: f32 = 5.0;
const GRAVITY: f32 = 0.3;

#[derive(Resource, Default)]
pub struct MovementInput {
    fmove: f32,
    smove: f32,
    jump: bool,
}

pub fn movement_input_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut movement: ResMut<MovementInput>,
    key_bindings: Res<KeyBindings>,
) {
    let mut raw_m = Vec2::new(0.0, 0.0);

    if input.pressed(key_bindings.move_forward) {
        raw_m.x += 1.0
    }

    if input.pressed(key_bindings.move_backward) {
        raw_m.x -= 1.0
    }

    if input.pressed(key_bindings.move_left) {
        raw_m.y += 1.0
    }

    if input.pressed(key_bindings.move_right) {
        raw_m.y -= 1.0
    }

    if input.just_pressed(key_bindings.jump) {
        movement.jump = true;
    }

    movement.fmove = raw_m.x;
    movement.smove = raw_m.y;
}

// TODO: need to restrict air movement

pub fn player_move(
    movement: Res<MovementInput>,
    camera_transform: Single<&Transform, With<Head>>,
    mut player_query: Query<(&mut LinearVelocity)>,
) {
    for mut real_lin_vel in player_query.iter_mut() {
        let (yaw_head, _, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        let lin_vel = Vec3::new(real_lin_vel.x, 0.0, real_lin_vel.z);

        // player looking direction
        let forward = Vec3::new(-yaw_head.sin(), 0.0, -yaw_head.cos()).normalize();
        let right = Vec3::new(-yaw_head.cos(), 0.0, yaw_head.sin()).normalize();

        let wish_velocity = Vec3::new(
            forward.x * movement.fmove + right.x * movement.smove,
            0.0,
            forward.z * movement.fmove + right.z * movement.smove,
        )
        .normalize_or_zero()
            * PLAYER_SPEED;

        // quake calc starting from here
        let mut wish_speed = wish_velocity.length();
        let wish_velocity = wish_velocity.normalize_or_zero();

        if wish_speed > PLAYER_SPEED {
            wish_speed = PLAYER_SPEED;
        }

        let current_speed = lin_vel.dot(wish_velocity);
        let add_speed = wish_speed - current_speed;

        if add_speed <= 0.0 {
            return;
        }

        let mut accel_speed = 0.3;
        if accel_speed > add_speed {
            accel_speed = add_speed
        }

        real_lin_vel.x += accel_speed * wish_velocity.x;
        real_lin_vel.z += accel_speed * wish_velocity.z;
    }
}

pub fn friction(mut player_query: Query<(&mut LinearVelocity, &Player)>) {
    for (mut lin_vel, player_promp) in player_query.iter_mut() {
        if player_promp.on_ground {
            let force = lin_vel.length();
            let fri_dir = -lin_vel.normalize_or_zero();

            if force <= 0.05 {
                lin_vel.x = 0.0;
                lin_vel.z = 0.0;
            } else {
                lin_vel.x += fri_dir.x * 0.1 * force;
                lin_vel.z += fri_dir.z * 0.1 * force;
            }
        }
    }
}

pub fn body_collision(collisions: Collisions, gc_entity: Single<Entity, With<Player>>) {
    for test in collisions.entities_colliding_with(*gc_entity) {}
}

pub fn ground_check(
    mut movement: ResMut<MovementInput>,
    raycasting: Query<(&RayCaster, &RayHits)>,
    mut player_promp: Single<(&mut Transform, &mut Player, &mut LinearVelocity)>,
) {
    for (_ray, hits) in &raycasting {
        if hits.is_empty() {
            player_promp.1.on_ground = false;
            player_promp.2.y -= GRAVITY;
        } else {
            player_promp.1.on_ground = true;
            player_promp.2.y = 0.0;

            if movement.jump {
                player_promp.2.y = 5.0;
            }

            for hit in hits.iter() {
                let angle = calculate_slope_angle(hit.normal);

                if angle < 16.0 {
                    player_promp.0.translation.y += 0.5 - hit.distance;
                }

                if angle > 45.0 {
                    // Calculate the component of gravity along the slope
                    let gravity = Vec3::new(0.0, -GRAVITY, 0.0);
                    let gravity_along_slope = gravity - hit.normal * hit.normal.dot(gravity);

                    // Apply sliding acceleration
                    let slide_acceleration = gravity_along_slope;
                    player_promp.2.x += slide_acceleration.x;
                    player_promp.2.y += slide_acceleration.y;
                    player_promp.2.z += slide_acceleration.z;
                }

                if hit.distance < 0.515 {
                    player_promp.0.translation.y += 0.5 - hit.distance;
                }
            }
        }
    }

    if movement.jump {
        movement.jump = false;
    }
}

fn calculate_slope_angle(normal: Vec3) -> f32 {
    let up = Vec3::Y;

    // Calculate angle between normal and up vector
    let dot_product = normal.dot(up);
    let angle_radians = dot_product.acos();

    let angle_degrees = angle_radians.to_degrees();

    angle_degrees
}

pub fn player_position_reset(
    key_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    if key_input.just_pressed(KeyCode::KeyX) {
        player_transform.translation = Vec3::new(0.0, 0.5, 0.0);
    }
}
