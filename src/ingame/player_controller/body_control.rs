use avian3d::{
    math::{Scalar, Vector},
    prelude::*,
};
use bevy::prelude::*;
use std::f32::consts::PI;

use super::{player::Player, KeyBindings};
use crate::ingame::{player::Head, player_controller::player::GroundChecker};

const PLAYER_SPEED: f32 = 5.0;
const PLAYER_ACCEL: f32 = 0.5;
const WALK_SPEED: f32 = 2.0;
const WALK_ACCEL: f32 = 0.3;
const FRICTION: f32 = 0.2;
const GRAVITY: f32 = 0.3;

pub fn player_position_reset(
    key_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<(&mut Transform, &mut LinearVelocity), With<Player>>,
) {
    if key_input.just_pressed(KeyCode::KeyX) {
        player.0.translation = Vec3::new(0.0, 0.5, 0.0);
        *player.1 = LinearVelocity::ZERO;
    }
}

pub fn movement_input_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Player>,
    key_bindings: Res<KeyBindings>,
) {
    let forward = input.pressed(key_bindings.move_forward);
    let backward = input.pressed(key_bindings.move_backward);
    let left = input.pressed(key_bindings.move_left);
    let right = input.pressed(key_bindings.move_right);

    player.fmove = forward as i8 - backward as i8;
    player.smove = left as i8 - right as i8;
    player.jump = input.pressed(key_bindings.jump);
    player.walk = input.pressed(key_bindings.walk);
}

pub fn player_move(
    camera_transform: Single<&Transform, With<Head>>,
    mut player_query: Query<(&mut LinearVelocity, &Player)>,
) {
    for (mut real_lin_vel, player) in player_query.iter_mut() {
        let (yaw_head, _, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        let mut lin_vel = Vec3::new(real_lin_vel.x, 0.0, real_lin_vel.z);

        // player looking direction
        let forward = Vec3::new(-yaw_head.sin(), 0.0, -yaw_head.cos()).normalize();
        let right = Vec3::new(-yaw_head.cos(), 0.0, yaw_head.sin()).normalize();

        let accel = if player.walk {
            WALK_ACCEL
        } else {
            PLAYER_ACCEL
        };

        let wish_dir = Vec3::new(
            forward.x * player.fmove as f32 + right.x * player.smove as f32,
            0.0,
            forward.z * player.fmove as f32 + right.z * player.smove as f32,
        )
        .normalize_or_zero()
            * accel;

        let final_dir: Vec3 = wish_dir + friction(lin_vel, lin_vel.length());

        lin_vel.x += final_dir.x;
        lin_vel.z += final_dir.z;

        if player.walk && lin_vel.length() > WALK_SPEED {
            lin_vel = lin_vel.normalize_or_zero() * WALK_SPEED
        } else if lin_vel.length() > PLAYER_SPEED {
            lin_vel = lin_vel.normalize_or_zero() * PLAYER_SPEED
        }

        if lin_vel.length() <= 0.1 {
            lin_vel.x = 0.0;
            lin_vel.z = 0.0;
        }

        real_lin_vel.x = lin_vel.x;
        real_lin_vel.z = lin_vel.z;
    }
}

pub fn friction(lin_vel: Vec3, force: f32) -> Vec3 {
    let mut fri_dir = -lin_vel.normalize_or_zero();

    let friction = if force / 2.0 > FRICTION {
        FRICTION
    } else {
        force / 2.0
    };

    fri_dir.x *= friction;
    fri_dir.z *= friction;

    fri_dir
}

pub fn apply_gravity(mut lin_vel: Single<&mut LinearVelocity, With<Player>>) {
    lin_vel.y -= GRAVITY;
}

pub fn kinematic_controller_collisions(
    time: Res<Time>,
    cg: Res<ContactGraph>,
    bodies: Query<&RigidBody>,
    player_entity: Single<Entity, With<Player>>,
    mut player_query: Query<(&mut Position, &mut LinearVelocity), With<Player>>,
) {
    for contact_pair in cg.contact_pairs_with(*player_entity) {
        let cp1 = contact_pair.collider1;
        let cp2 = contact_pair.collider2;

        let is_first: bool;
        let is_other_dynamic: bool;

        let (mut position, mut lin_vel) = if let Ok(character) = player_query.get_mut(cp1) {
            is_first = true;
            is_other_dynamic = bodies.get(cp2).is_ok_and(|rb| rb.is_dynamic());
            character
        } else if let Ok(character) = player_query.get_mut(cp2) {
            is_first = false;
            is_other_dynamic = bodies.get(cp1).is_ok_and(|rb| rb.is_dynamic());
            character
        } else {
            continue;
        };

        for manifold in contact_pair.manifolds.iter() {
            let normal = if is_first {
                -manifold.normal
            } else {
                manifold.normal
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            let mut auto_step: Option<f32> = None;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.points.iter() {
                let point = if is_first {
                    contact.anchor1
                } else {
                    contact.anchor2
                };

                if point.y < -0.25 && point.y > -0.5 {
                    //info!("point y = {}", point.y);
                    if let Some(pt) = auto_step {
                        if pt < point.y {
                            auto_step = Some(point.y);
                        }
                    } else {
                        auto_step = Some(point.y);
                    }
                } else {
                    auto_step = None;
                }

                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let max_slope_angle: Scalar = PI * 0.25;
            let slope_angle = normal.angle_between(Vector::Y);
            let climbable = slope_angle.abs() <= max_slope_angle;

            if deepest_penetration > 0.0 {
                if climbable {
                    lin_vel.y = 0.0;
                    if let Some(height) = auto_step {
                        position.y += 0.5 + height;
                    }

                    // Points in the normal's direction in the XZ plane.
                    let normal_direction_xz =
                        normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // info!("test = {}", normal_direction_xz);

                    // The movement speed along the direction above.
                    // let linear_velocity_xz = lin_vel.dot(normal_direction_xz);
                    // let max_y_speed = -linear_velocity_xz * slope_angle.tan();
                    // lin_vel.y = lin_vel.y.max(max_y_speed);
                } else {
                    if let Some(height) = auto_step {
                        lin_vel.y = 0.0;

                        info!("pos y = {}", position.y);

                        if height < -0.45 {
                            position.y += 0.5 + height;
                        } else {
                            position.y += (0.5 + height) / 2.0;
                        }
                    } else {
                        position.0 += normal * deepest_penetration;

                        // Don't apply an impulse if the character is moving away from the surface.
                        if lin_vel.dot(normal) > 0.0 {
                            continue;
                        }

                        // Slide along the surface, rejecting the velocity along the contact normal.
                        //let impulse = lin_vel.reject_from_normalized(normal);
                        //lin_vel.0 = impulse;
                    }
                }
            }

            /*
            else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = lin_vel.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude = normal_speed - (deepest_penetration / time.delta_secs());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    lin_vel.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    lin_vel.0 -= impulse;
                }
            }
             */
        }
    }
}
