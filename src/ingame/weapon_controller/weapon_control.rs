use avian3d::parry::math::Rotation;
use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseMotion},
    math::VectorSpace,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use super::{WeaponPromp, WeaponReloadingEvent, WeaponShootingEvent, WeaponState};
use crate::ingame::{player::Head, GameSettings};

pub fn camera_recoil(
    time: Res<Time>,
    settings: ResMut<GameSettings>,
    mut event_reader: EventReader<WeaponShootingEvent>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    mut head_query: Query<&mut Transform, With<Head>>,
) {
    let Projection::Perspective(persp) = camera_query.single_mut().into_inner() else {
        return;
    };

    for _event in event_reader.read() {
        let mut head_transform = head_query.single_mut();
        let (mut yaw_camera, mut pitch_camera, _) = head_transform.rotation.to_euler(EulerRot::YXZ);

        pitch_camera += 0.015;
        yaw_camera += thread_rng().gen_range(-0.005..0.005);

        pitch_camera = pitch_camera.clamp(-PI / 2.0, PI / 2.0);
        head_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera)
            * Quat::from_axis_angle(Vec3::X, pitch_camera);

        persp
            .fov
            .smooth_nudge(&(persp.fov + (10.0 / 180.0 * PI)), 50.0, time.delta_secs());
    }

    if settings.fov < (persp.fov / PI * 180.0) {
        persp
            .fov
            .smooth_nudge(&(settings.fov / 180.0 * PI), 20.0, time.delta_secs());
    }
}

// FIXME: fix this shit
pub fn sway_weapon(
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut weapon_transform: Single<&mut Transform, With<WeaponPromp>>,
) {
    let (mut weapon_rot_y, mut weapon_rot_x, _) = weapon_transform.rotation.to_euler(EulerRot::YXZ);

    let delta = accumulated_mouse_motion.delta;

    if delta == Vec2::ZERO {
        weapon_rot_x = 0.;
        weapon_rot_y = 0.;
    } else {
        weapon_rot_y += delta.x / 2000.;
        weapon_rot_x += delta.y / 2000.;

        info!("rot = {}", weapon_rot_y);
    }

    weapon_transform.rotation.smooth_nudge(
        &(Quat::from_axis_angle(Vec3::Y, weapon_rot_y.clamp(-0.03, 0.03))
            * Quat::from_axis_angle(Vec3::X, weapon_rot_x.clamp(-0.03, 0.03))),
        20.0,
        time.delta_secs(),
    );
}

pub fn scoped_sway_weapon(
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut weapon_query: Query<&mut Transform, With<WeaponPromp>>,
    mut mouse_event: EventReader<MouseMotion>,
) {
    for window in primary_window.iter() {
        let mut weapon_transform = weapon_query.single_mut();
        let (mut weapon_rot_y, mut weapon_rot_x, _) =
            weapon_transform.rotation.to_euler(EulerRot::YXZ);

        if mouse_event.is_empty() {
            if weapon_rot_x.abs() > 0.05 {
                weapon_rot_x /= 5000. * time.delta_secs();
            } else {
                weapon_rot_x = 0.0;
            }

            if weapon_rot_y.abs() > 0.05 {
                weapon_rot_y /= 5000. * time.delta_secs();
            } else {
                weapon_rot_y = 0.0;
            }
        }

        for motion in mouse_event.read() {
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    weapon_rot_y +=
                        (motion.delta.x - weapon_rot_y * 900.) * time.delta_secs() * 0.1;
                    weapon_rot_x +=
                        (motion.delta.y - weapon_rot_x * 900.) * time.delta_secs() * 0.1;
                }
            }
        }

        weapon_transform.rotation = Quat::from_axis_angle(Vec3::Y, weapon_rot_y)
            * Quat::from_axis_angle(Vec3::X, weapon_rot_x);
    }
}

pub fn shooting_sound(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    weapon_state: Res<State<WeaponState>>,
    mut shot_event_reader: EventReader<WeaponShootingEvent>,
    mut reload_event_reader: EventReader<WeaponReloadingEvent>,
) {
    // shooting sound
    for _event in shot_event_reader.read() {
        match weapon_state.get() {
            WeaponState::P226 => audio.play(asset_server.load("sounds/p226_shot.ogg")),
            WeaponState::AK15 => audio.play(asset_server.load("sounds/ak15_shot1.ogg")),
        };
    }

    // reloading sound
    for _event in reload_event_reader.read() {
        match weapon_state.get() {
            WeaponState::P226 => audio.play(asset_server.load("sounds/p226_reload.ogg")),
            WeaponState::AK15 => audio.play(asset_server.load("sounds/ak15_reload.ogg")),
        };
    }

    // raising sound
    // lowering sound
}
