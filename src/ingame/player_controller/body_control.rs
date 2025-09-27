use avian3d::{math::Scalar, prelude::*};
use bevy::prelude::*;

use crate::ingame::player::Head;

use super::{player::Player, KeyBindings};

const PLAYER_SPEED: f32 = 5.0;
const PLAYER_ACCEL: f32 = 0.5;
const WALK_SPEED: f32 = 2.0;
const WALK_ACCEL: f32 = 0.3;
const FRICTION: f32 = 0.2;
const GRAVITY: f32 = 0.3;

pub fn movement_input_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Player>,
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

    if input.pressed(key_bindings.jump) {
        player.jump = true;
    }

    if input.pressed(key_bindings.walk) {
        player.walk = true;
    } else {
        player.walk = false;
    }

    player.fmove = raw_m.x;
    player.smove = raw_m.y;
}

// TODO: need to restrict air movement

pub fn player_move(
    camera_transform: Single<&Transform, With<Head>>,
    mut player_query: Query<(&mut LinearVelocity, &Player)>,
) {
    for (mut real_lin_vel, player_promp) in player_query.iter_mut() {
        let (yaw_head, _, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        let mut lin_vel = Vec3::new(real_lin_vel.x, 0.0, real_lin_vel.z);

        // player looking direction
        let forward = Vec3::new(-yaw_head.sin(), 0.0, -yaw_head.cos()).normalize();
        let right = Vec3::new(-yaw_head.cos(), 0.0, yaw_head.sin()).normalize();

        let accel = if player_promp.walk {
            WALK_ACCEL
        } else {
            PLAYER_ACCEL
        };

        let wish_dir = Vec3::new(
            forward.x * player_promp.fmove + right.x * player_promp.smove,
            0.0,
            forward.z * player_promp.fmove + right.z * player_promp.smove,
        )
        .normalize_or_zero()
            * accel;

        let final_dir: Vec3 = match player_promp.on_ground {
            true => wish_dir + friction(lin_vel, lin_vel.length()),
            false => wish_dir,
        };

        lin_vel.x += final_dir.x;
        lin_vel.z += final_dir.z;

        if player_promp.walk && lin_vel.length() > WALK_SPEED {
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

pub fn body_collision(
    contact_graph: Res<ContactGraph>,
    player_entity: Single<Entity, With<Player>>,
) {
    for contact_pair in contact_graph.collisions_with(*player_entity) {
        // Get contact points
        for manifold in &contact_pair.manifolds {
            for contact_point in &manifold.points {
                let point1 = contact_point;
                let normal = manifold.normal;

                // println!("contact point = {:?}", point1.local_point1);
                // println!("contact normal = {:?}", normal);
            }
        }
    }
}

pub fn ground_check(
    raycasting: Query<(&RayCaster, &RayHits)>,
    player_query: Single<(&mut Transform, &mut Player, &mut LinearVelocity)>,
) {
    let (mut player_trs, mut player_promp, mut lin_vel) = player_query.into_inner();

    for (_ray, hits) in &raycasting {
        if hits.is_empty() {
            player_promp.on_ground = false;
            lin_vel.y -= GRAVITY;
        } else {
            player_promp.on_ground = true;
            lin_vel.y = 0.0;

            if player_promp.jump {
                lin_vel.y = 5.0;
            }

            for hit in hits.iter() {
                let angle = calculate_slope_angle(hit.normal);

                if angle <= 45.0 {
                    player_trs.translation.y += 0.5 - hit.distance;
                } else if angle > 45.0 {
                    // Calculate the component of gravity along the slope
                    let gravity = Vec3::new(0.0, -GRAVITY, 0.0);
                    let gravity_along_slope = gravity - hit.normal * hit.normal.dot(gravity);

                    // Apply sliding acceleration
                    let slide_acceleration = gravity_along_slope;
                    lin_vel.x += slide_acceleration.x;
                    lin_vel.y += slide_acceleration.y;
                    lin_vel.z += slide_acceleration.z;
                }

                // if hit.distance < 0.515 {
                //    player_trs.translation.y += (0.5 - hit.distance) / 500000.0;
                // }
            }
        }
    }

    if player_promp.jump {
        player_promp.jump = false;
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
    mut player: Single<(&mut Transform, &mut LinearVelocity), With<Player>>,
) {
    if key_input.just_pressed(KeyCode::KeyX) {
        player.0.translation = Vec3::new(0.0, 0.5, 0.0);
        *player.1 = LinearVelocity::ZERO;
    }
}
