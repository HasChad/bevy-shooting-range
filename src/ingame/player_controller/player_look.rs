use std::f32::consts::PI;

use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use super::{Head, Player};
use crate::ingame::GameSettings;

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

pub fn player_look(
    settings: Res<GameSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query_camera: Query<&mut Transform, With<Head>>,
    mut query_player: Query<&mut Transform, (With<Player>, Without<Head>)>,
) {
    // ! point
    if let Ok(window) = primary_window.get_single() {
        for ev in state.reader_motion.read(&motion) {
            let mut camera_transform = query_camera.single_mut();
            let mut player_transform = query_player.single_mut();

            let (mut yaw_camera, mut pitch_camera, _) =
                camera_transform.rotation.to_euler(EulerRot::YXZ);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    pitch_camera -=
                        (settings.sensitivity / 10000.0 * ev.delta.y * window_scale).to_radians();
                    yaw_camera -=
                        (settings.sensitivity / 10000.0 * ev.delta.x * window_scale).to_radians();
                }
            }

            pitch_camera = pitch_camera.clamp(-PI / 2.0, PI / 2.0);

            // Order is important to prevent unintended roll

            camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera)
                * Quat::from_axis_angle(Vec3::X, pitch_camera);
            player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera);
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

pub fn camera_follow_player(
    mut query_camera: Query<&mut Transform, With<Head>>,
    query_player: Query<&Transform, (With<Player>, Without<Head>)>,
) {
    for mut camera_transform in query_camera.iter_mut() {
        for player_transform in query_player.iter() {
            camera_transform.translation = player_transform.translation;
            camera_transform.translation.y = player_transform.translation.y + 0.5;
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
