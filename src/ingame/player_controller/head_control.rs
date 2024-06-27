use bevy::{input::mouse::MouseMotion, prelude::*};
use std::f32::consts::PI;

use super::{Head, Player};
use crate::ingame::GameSettings;

pub fn player_look(
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut mouse_event: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Head>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Head>)>,
) {
    for motion in mouse_event.read() {
        let mut camera_transform = camera_query.single_mut();
        let mut player_transform = player_query.single_mut();

        let (mut yaw_camera, mut pitch_camera, _) =
            camera_transform.rotation.to_euler(EulerRot::YXZ);

        pitch_camera -=
            (settings.sensitivity * 20.0 * motion.delta.y * time.delta_seconds()).to_radians();
        yaw_camera -=
            (settings.sensitivity * 20.0 * motion.delta.x * time.delta_seconds()).to_radians();

        pitch_camera = pitch_camera.clamp(-PI / 2.0, PI / 2.0);

        // Order is important to prevent unintended roll
        camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera)
            * Quat::from_axis_angle(Vec3::X, pitch_camera);
        player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera);
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
