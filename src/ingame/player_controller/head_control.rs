use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

use super::{Head, Player};
use crate::ingame::GameSettings;

pub fn player_look(
    settings: Res<GameSettings>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera_query: Query<&mut Transform, With<Head>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Head>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut player_transform = player_query.single_mut();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * settings.sensitivity / 300.0;
        let delta_pitch = -delta.y * settings.sensitivity / 300.0;

        let (yaw, pitch, roll) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta_yaw;
        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw);
    }
}

pub fn camera_follow_player(
    mut camera_query: Query<&mut Transform, With<Head>>,
    player_query: Query<&Transform, (With<Player>, Without<Head>)>,
) {
    for player_transform in player_query.iter() {
        for mut camera_transform in camera_query.iter_mut() {
            camera_transform.translation = player_transform.translation;
            camera_transform.translation.y = player_transform.translation.y + 0.25;
            //for inspecting player collider
            //camera_transform.translation.z = player_transform.translation.z + 1.;
        }
    }
}
