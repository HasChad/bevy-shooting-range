use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use std::f32::consts::PI;

use crate::ingame::GameSettings;
use crate::{Head, Player};

pub fn player_look(
    time: Res<Time>,
    settings: Res<GameSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_event: EventReader<MouseMotion>,
    mut query_camera: Query<&mut Transform, With<Head>>,
    mut query_player: Query<&mut Transform, (With<Player>, Without<Head>)>,
) {
    for window in primary_window.iter() {
        for motion in mouse_event.read() {
            let mut camera_transform = query_camera.single_mut();
            let mut player_transform = query_player.single_mut();

            let (mut yaw_camera, mut pitch_camera, _) =
                camera_transform.rotation.to_euler(EulerRot::YXZ);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    pitch_camera -=
                        (settings.sensitivity * 20.0 * motion.delta.y * time.delta_seconds())
                            .to_radians();
                    yaw_camera -=
                        (settings.sensitivity * 20.0 * motion.delta.x * time.delta_seconds())
                            .to_radians();
                }
            }
            pitch_camera = pitch_camera.clamp(-PI / 2.0, PI / 2.0);

            // Order is important to prevent unintended roll
            camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera)
                * Quat::from_axis_angle(Vec3::X, pitch_camera);
            player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera);
        }
    }
}

pub fn camera_follow_player(
    mut query_camera: Query<&mut Transform, With<Head>>,
    query_player: Query<&Transform, (With<Player>, Without<Head>)>,
) {
    for player_transform in query_player.iter() {
        for mut camera_transform in query_camera.iter_mut() {
            camera_transform.translation = player_transform.translation;
            camera_transform.translation.y = player_transform.translation.y + 0.25;
            //for inspecting player collider
            //camera_transform.translation.z = player_transform.translation.z + 1.;
        }
    }
}

pub fn edit_mode_toggler(input: ResMut<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if input.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        match window.cursor.grab_mode {
            CursorGrabMode::Confined => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
            }
        }
    }
}
