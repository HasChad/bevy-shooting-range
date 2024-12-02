use bevy::{
    animation::RepeatAnimation,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};
use std::{f32::consts::PI, time::Duration};

use super::{
    weapons::{ReloadingAnimations, ShootingAnimations},
    WeaponPromp, WeaponReloadingEvent, WeaponShootingEvent, WeaponState,
};
use crate::ingame::{
    player::{self, Head},
    GameSettings,
};

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

        persp.fov += 3.0 / 180.0 * PI;
    }

    if settings.fov < (persp.fov / PI * 180.0) {
        persp.fov -= (50.0 / 180.0 * PI) * time.delta_secs();
    }
}

pub fn sway_weapon(
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
                weapon_rot_x /= 15000. * time.delta_secs();
            } else {
                weapon_rot_x = 0.0;
            }

            if weapon_rot_y.abs() > 0.05 {
                weapon_rot_y /= 15000. * time.delta_secs();
            } else {
                weapon_rot_y = 0.0;
            }
        }

        for motion in mouse_event.read() {
            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    weapon_rot_y +=
                        (motion.delta.x - weapon_rot_y * 300.) * time.delta_secs() * 0.1;
                    weapon_rot_x +=
                        (motion.delta.y - weapon_rot_x * 300.) * time.delta_secs() * 0.1;
                }
            }
        }

        weapon_transform.rotation = Quat::from_axis_angle(Vec3::Y, weapon_rot_y)
            * Quat::from_axis_angle(Vec3::X, weapon_rot_x);
    }
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

pub fn weapon_animation(
    weapon_state: Res<State<WeaponState>>,
    mut shot_event_reader: EventReader<WeaponShootingEvent>,
    mut reload_event_reader: EventReader<WeaponReloadingEvent>,
    shot_anim: Res<ShootingAnimations>,
    reload_anim: Res<ReloadingAnimations>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        for _event in shot_event_reader.read() {
            let anim = match weapon_state.get() {
                WeaponState::P226 => shot_anim.animations[0],
                WeaponState::AK15 => shot_anim.animations[1],
            };

            transitions.play(&mut player, anim, Duration::ZERO);
        }

        for _event in reload_event_reader.read() {
            let anim = match weapon_state.get() {
                WeaponState::P226 => reload_anim.animations[0],
                WeaponState::AK15 => reload_anim.animations[1],
            };

            transitions.play(&mut player, anim, Duration::ZERO);
        }
    }
}

pub fn setup_scene_once_loaded(
    mut commands: Commands,
    shot_anim: Res<ShootingAnimations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, _player) in &mut players {
        let transitions = AnimationTransitions::new();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(shot_anim.graph.clone()))
            .insert(transitions);
    }
}

/*
// shooting animation
    for _event in shot_event_reader.read() {
        for mut animation_player in &mut animation_player_query {
            match weapon_state.get() {
                WeaponState::P226 => animation_player.play(shot_anim.0[0].clone_weak()),
                WeaponState::AK15 => animation_player.play(shot_anim.0[1].clone_weak()),
                WeaponState::FNFAL => animation_player.play(shot_anim.0[2].clone_weak()),
                WeaponState::MSR => animation_player.play(shot_anim.0[3].clone_weak()), //FIXME: need msr animation
            };

            animation_player.set_repeat(RepeatAnimation::Count(1));
            animation_player.replay();
        }
    }
 */
