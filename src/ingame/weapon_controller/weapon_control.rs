#![allow(clippy::too_many_arguments)]

use bevy::{
    animation::RepeatAnimation,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use super::{WeaponActionState, WeaponPromp, WeaponShootingEvent, WeaponState};
use crate::ingame::{
    crosshair::{CrosshairLine, CrosshairLineSettings},
    player::Head,
    GameSettings, KeyBindings, ShootingAnimations,
};

#[derive(Resource)]
pub struct LerpTimer {
    scope_timer: Timer,
    //recoil_timer: Timer,
}

impl Default for LerpTimer {
    fn default() -> Self {
        LerpTimer {
            scope_timer: Timer::from_seconds(0.1, TimerMode::Once),
            //recoil_timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
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
            if weapon_rot_x.abs() > 0.0 {
                weapon_rot_x /= 15000. * time.delta_seconds();
            }
            if weapon_rot_y.abs() > 0.0 {
                weapon_rot_y /= 15000. * time.delta_seconds();
            }
        }

        for motion in mouse_event.read() {
            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    weapon_rot_y +=
                        (motion.delta.x - weapon_rot_y * 300.) * time.delta_seconds() * 0.1;
                    weapon_rot_x +=
                        (motion.delta.y - weapon_rot_x * 300.) * time.delta_seconds() * 0.1;
                }
            }
        }

        weapon_transform.rotation = Quat::from_axis_angle(Vec3::Y, weapon_rot_y)
            * Quat::from_axis_angle(Vec3::X, weapon_rot_x);
    }
}

pub fn shooting_event(
    key_bindings: Res<KeyBindings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<WeaponShootingEvent>,
    mut weapon_query: Query<&mut WeaponPromp>,
    mut windows: Query<&mut Window>,
    mut next_state: ResMut<NextState<WeaponActionState>>,
) {
    for mut window in windows.iter_mut() {
        if window.cursor.grab_mode == CursorGrabMode::Confined {
            //Center mouse becasuse confined mod is not working on Windows right now
            let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            window.set_cursor_position(Some(center));

            for mut weapon_promp in weapon_query.iter_mut() {
                if weapon_promp.okay_to_shoot {
                    //semi auto shot
                    if mouse_input.just_pressed(key_bindings.fire) && !weapon_promp.is_auto {
                        weapon_promp.mag_capacity -= 1;
                        event_writer.send(WeaponShootingEvent);
                        weapon_promp.okay_to_shoot = false;
                    }
                    //full auto shot
                    if mouse_input.pressed(key_bindings.fire) && weapon_promp.is_auto {
                        weapon_promp.mag_capacity -= 1;
                        event_writer.send(WeaponShootingEvent);
                        weapon_promp.okay_to_shoot = false;
                    }
                }
                //reload
                if (weapon_promp.mag_capacity == 0
                    || (keyboard_input.just_pressed(key_bindings.reload))
                        && weapon_promp.mag_capacity < weapon_promp.self_mag_cap())
                    && weapon_promp.ammo_capacity > 0
                {
                    next_state.set(WeaponActionState::Reloading)
                }
            }
        }
    }
}

pub fn firerate_timer(mut weapon_query: Query<&mut WeaponPromp>, time: Res<Time>) {
    for mut weapon_promp in weapon_query.iter_mut() {
        if !weapon_promp.okay_to_shoot {
            weapon_promp.firerate.tick(time.delta());

            if weapon_promp.firerate.finished() {
                weapon_promp.firerate.reset();
                weapon_promp.okay_to_shoot = true;
            }
        }
    }
}

pub fn reload_timer(
    mut weapon_query: Query<&mut WeaponPromp>,
    mut next_state: ResMut<NextState<WeaponActionState>>,
    time: Res<Time>,
) {
    for mut weapon_promp in weapon_query.iter_mut() {
        weapon_promp.reload_timer.tick(time.delta());

        if weapon_promp.reload_timer.finished() {
            weapon_promp.reload_timer.reset();
            weapon_promp.ammo_capacity -= weapon_promp.self_mag_cap() - weapon_promp.mag_capacity;
            weapon_promp.mag_capacity = weapon_promp.self_mag_cap();
            next_state.set(WeaponActionState::Shooting)
        }
    }
}

pub fn shooting_camera_shake(
    mut event_reader: EventReader<WeaponShootingEvent>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    settings: ResMut<GameSettings>,
    time: Res<Time>,
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
        persp.fov -= (50.0 / 180.0 * PI) * time.delta_seconds();
    }
}

pub fn scope(
    time: Res<Time>,
    settings: ResMut<GameSettings>,
    key_bindings: Res<KeyBindings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut lerp_timer: ResMut<LerpTimer>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    mut weapon_query: Query<&mut Transform, With<WeaponPromp>>,
    mut crosshair_query: Query<&mut Visibility, With<CrosshairLine>>,
    crosshair_settings: Res<CrosshairLineSettings>,
) {
    let mut weapon_transform = weapon_query.single_mut();
    let Projection::Perspective(persp) = camera_query.single_mut().into_inner() else {
        return;
    };

    if mouse_input.just_pressed(key_bindings.scope) || mouse_input.just_released(key_bindings.scope)
    {
        lerp_timer.scope_timer.reset()
    }
    if mouse_input.pressed(key_bindings.scope) {
        lerp_timer.scope_timer.tick(time.delta());

        for mut croshair_visib in crosshair_query.iter_mut() {
            *croshair_visib = Visibility::Hidden;
        }

        let percentage_complete =
            lerp_timer.scope_timer.elapsed_secs() / lerp_timer.scope_timer.duration().as_secs_f32();

        persp.fov = persp
            .fov
            .lerp((settings.fov) * 0.6 / 180.0 * PI, percentage_complete);

        weapon_transform.translation = weapon_transform
            .translation
            .lerp(Vec3::new(0.0, 0.0, -0.15), percentage_complete);
    } else if !lerp_timer.scope_timer.finished() {
        lerp_timer.scope_timer.tick(time.delta());

        for mut croshair_visib in crosshair_query.iter_mut() {
            *croshair_visib = crosshair_settings.enable;
        }

        let percentage_complete =
            lerp_timer.scope_timer.elapsed_secs() / lerp_timer.scope_timer.duration().as_secs_f32();

        persp.fov = persp
            .fov
            .lerp(settings.fov / 180.0 * PI, percentage_complete);

        weapon_transform.translation = weapon_transform
            .translation
            .lerp(Vec3::new(0.075, -0.04, -0.1), percentage_complete);
    }
}

pub fn shooting_sound(
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<WeaponShootingEvent>,
    weapon_state: Res<State<WeaponState>>,
) {
    for _event in event_reader.read() {
        match weapon_state.get() {
            WeaponState::P226 => audio.play(asset_server.load("sounds/p226_shot.ogg")),
            WeaponState::AK15 => audio.play(asset_server.load("sounds/ak15_shot.ogg")),
            WeaponState::FNFAL => audio.play(asset_server.load("sounds/fal_shot.ogg")),
            WeaponState::MSR => audio.play(asset_server.load("sounds/msr_shot.ogg")), //FIXME: neeed msr sound
        };
    }
}

pub fn weapon_animation_setup(
    shot_anim: Res<ShootingAnimations>,
    weapon_state: Res<State<WeaponState>>,
    mut animation_player_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut animation_player in &mut animation_player_query {
        match weapon_state.get() {
            WeaponState::P226 => animation_player.play(shot_anim.0[0].clone_weak()).repeat(),
            WeaponState::AK15 => animation_player.play(shot_anim.0[1].clone_weak()).repeat(),
            WeaponState::FNFAL => animation_player.play(shot_anim.0[2].clone_weak()).repeat(),
            WeaponState::MSR => animation_player.play(shot_anim.0[0].clone_weak()).repeat(), //FIXME: neeed msr animation
        };
        animation_player.set_repeat(RepeatAnimation::Count(0));
    }
}

pub fn weapon_play_animation(
    mut event_reader: EventReader<WeaponShootingEvent>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
) {
    for _event in event_reader.read() {
        for mut animation_player in &mut animation_player_query {
            animation_player.set_repeat(RepeatAnimation::Count(1));
            animation_player.replay();
        }
    }
}
