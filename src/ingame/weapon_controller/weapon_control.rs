use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use super::{WeaponPromp, WeaponShootingEvent};
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

pub fn sway_weapon(
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut weapon_transform: Single<&mut Transform, With<WeaponPromp>>,
) {
    let delta = accumulated_mouse_motion.delta;

    weapon_transform.rotation.smooth_nudge(
        &(Quat::from_axis_angle(Vec3::Y, (delta.x / 500.).clamp(-0.03, 0.03))
            * Quat::from_axis_angle(Vec3::X, (delta.y / 500.).clamp(-0.03, 0.03))),
        10.0,
        time.delta_secs(),
    );
}

pub fn scoped_sway_weapon(
    time: Res<Time>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut weapon_transform: Single<&mut Transform, With<WeaponPromp>>,
) {
    let delta = accumulated_mouse_motion.delta;

    weapon_transform.rotation.smooth_nudge(
        &(Quat::from_axis_angle(Vec3::Y, (delta.x / 1000.).clamp(-0.01, 0.01))
            * Quat::from_axis_angle(Vec3::X, (delta.y / 1000.).clamp(-0.01, 0.01))),
        10.0,
        time.delta_secs(),
    );
}
