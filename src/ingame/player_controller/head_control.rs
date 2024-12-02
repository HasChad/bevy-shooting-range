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
    mut camera: Single<&mut Transform, With<Head>>,
    player: Single<&Transform, (With<Player>, Without<Head>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, z } = player.translation;
    let direction = Vec3::new(x, y + 0.25, z); // + 1.0 when inspecting player collider

    camera
        .translation
        .smooth_nudge(&direction, 50.0, time.delta_secs());
}
