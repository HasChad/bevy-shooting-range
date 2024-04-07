use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use super::Player;
use crate::ingame::GameSettings;

#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
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

pub fn player_look(
    time: Res<Time>,
    settings: Res<GameSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query_camera: Query<&mut Transform, With<Camera3d>>,
    mut query_player: Query<&mut Transform, (With<Player>, Without<Camera3d>)>,
) {
    if let Ok(window) = primary_window.get_single() {
        for ev in state.reader_motion.read(&motion) {
            let mut camera_transform = query_camera.single_mut();
            let mut player_transform = query_player.single_mut();

            let (mut yaw_player, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
            let (_, mut pitch_camera, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    pitch_camera -= (settings.sensitivity * ev.delta.y * window_scale).to_radians()
                        * time.delta_seconds();
                    yaw_player -= (settings.sensitivity * ev.delta.x * window_scale).to_radians()
                        * time.delta_seconds();
                }
            }

            pitch_camera = pitch_camera.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_player);
            camera_transform.rotation = Quat::from_axis_angle(Vec3::X, pitch_camera);
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}
