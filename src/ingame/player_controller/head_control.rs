use std::f32::consts::PI;

use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use super::{Head, Player};
use crate::ingame::{GameSettings, WeaponChangeState, WeaponPromp};

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
    if let Ok(window) = primary_window.get_single() {
        for ev in state.reader_motion.read(&motion) {
            let mut camera_transform = query_camera.single_mut();
            let mut player_transform = query_player.single_mut();

            let (mut yaw_camera, mut pitch_camera, _) =
                camera_transform.rotation.to_euler(EulerRot::YXZ);

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    pitch_camera -= (settings.sensitivity / 10.0 * ev.delta.y).to_radians();
                    yaw_camera -= (settings.sensitivity / 10.0 * ev.delta.x).to_radians();
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

pub fn change_weapon(
    mut weapon_query: Query<(&mut WeaponPromp, &mut Handle<Scene>)>,
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut weapon_state: ResMut<NextState<WeaponChangeState>>,
) {
    for key in input.get_just_pressed() {
        for (mut weapon_promp, mut weapon_scene) in weapon_query.iter_mut() {
            let key = *key;
            match key {
                KeyCode::Digit1 => {
                    *weapon_promp = WeaponPromp::p226();
                    *weapon_scene = asset_server.load("models/P226.glb#Scene0");
                    weapon_state.set(WeaponChangeState::P226);
                }
                KeyCode::Digit2 => {
                    *weapon_promp = WeaponPromp::ak15();
                    *weapon_scene = asset_server.load("models/AK15.glb#Scene0");
                    weapon_state.set(WeaponChangeState::AK15);
                }
                KeyCode::Digit3 => {
                    *weapon_promp = WeaponPromp::msr();
                    *weapon_scene = asset_server.load("models/MSR.glb#Scene0");
                    weapon_state.set(WeaponChangeState::MSR);
                }
                _ => (),
            }
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
