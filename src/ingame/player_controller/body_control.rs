use avian3d::prelude::*;
use bevy::prelude::*;

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

        let mut real_lin_vel = Vec3::new(lin_vel.x, 0.0, lin_vel.z);

        if real_lin_vel.length() < 0.3 {
            lin_vel.z = 0.0;
            lin_vel.x = 0.0;
        }

        if player_promp.on_ground && real_lin_vel.length() > 0.0 {
            friction(real_lin_vel, &mut **lin_vel);
        }

        if wish_vel.length() > 0.0 {
            lin_vel.x += wish_vel.x * 1.0;
            lin_vel.z += wish_vel.z * 1.0;
        }

        real_lin_vel = Vec3::new(lin_vel.x, 0.0, lin_vel.z);

        if real_lin_vel.length() > settings.player_speed {
            let norm_lin_vel = real_lin_vel.normalize_or_zero();

            lin_vel.x = norm_lin_vel.x * settings.player_speed;
            lin_vel.z = norm_lin_vel.z * settings.player_speed;
        }
    }
}

fn friction(real_lin_vel: Vec3, lin_vel: &mut Vec3) {
    let mut fri_dir = -real_lin_vel.normalize_or_zero();

    if fri_dir.x < 0.00001 && fri_dir.x > -0.00001 {
        fri_dir.x = 0.0
    }

    if fri_dir.z < 0.00001 && fri_dir.z > -0.00001 {
        fri_dir.z = 0.0
    }

    if real_lin_vel.length() >= 0.5 {
        lin_vel.x += fri_dir.x * 0.2;
        lin_vel.z += fri_dir.z * 0.2;
    } else if real_lin_vel.length() > 0.3 {
        lin_vel.x += fri_dir.x * 0.2;
        lin_vel.z += fri_dir.z * 0.2;
    }
}

pub fn jumping(
    key_bindings: Res<KeyBindings>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut LinearVelocity, &Player)>,
) {
    for (mut lin_vel, player_promp) in player_query.iter_mut() {
        if player_promp.on_ground {
            if input.pressed(key_bindings.jump) {
                lin_vel.y = 5.0
            }
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
