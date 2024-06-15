use bevy::{animation::RepeatAnimation, prelude::*, window::CursorGrabMode};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use super::{player::Head, GameSettings};
use crate::ingame::Animations;

#[derive(Event)]
pub struct WeaponShootingEvent;

#[derive(Event)]
pub struct WeaponReloadingEvent;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum WeaponState {
    #[default]
    Shooting,
    Reloading,
}

#[derive(Resource)]
pub struct LerpTimer {
    timer: Timer,
}

impl Default for LerpTimer {
    fn default() -> Self {
        LerpTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct WeaponPromp {
    pub name: String,
    pub mag_capacity: u8,
    pub ammo_capacity: u8,
    pub head_damage: u8,
    pub body_damage: u8,
    pub is_auto: bool,
    pub okay_to_shoot: bool,
    pub firerate: Timer,
    pub reload_timer: Timer,
    //pub time_to_aim: Timer,
}

impl WeaponPromp {
    pub fn p226() -> WeaponPromp {
        WeaponPromp {
            name: "P226".to_owned(),
            mag_capacity: 15,
            ammo_capacity: 60,
            head_damage: 3,
            body_damage: 1,
            is_auto: false,
            okay_to_shoot: true,
            firerate: Timer::from_seconds(0.1, TimerMode::Once),
            reload_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }

    pub fn ak15() -> WeaponPromp {
        WeaponPromp {
            name: "AK-15".to_owned(),
            mag_capacity: 30,
            ammo_capacity: 120,
            head_damage: 10,
            body_damage: 4,
            is_auto: true,
            okay_to_shoot: true,
            firerate: Timer::from_seconds(0.08, TimerMode::Once),
            reload_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }

    pub fn msr() -> WeaponPromp {
        WeaponPromp {
            name: "MSR".to_owned(),
            mag_capacity: 5,
            ammo_capacity: 20,
            head_damage: 20,
            body_damage: 7,
            is_auto: false,
            okay_to_shoot: true,
            firerate: Timer::from_seconds(1.5, TimerMode::Once),
            reload_timer: Timer::from_seconds(2.5, TimerMode::Once),
        }
    }

    pub fn self_mag_cap(&self) -> u8 {
        match self.name.as_str() {
            "P226" => WeaponPromp::p226().mag_capacity,
            "AK-15" => WeaponPromp::ak15().mag_capacity,
            "MSR" => WeaponPromp::msr().mag_capacity,
            _ => panic!("No gun found for self_mag_cap"),
        }
    }
}

pub fn shooting_event(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<WeaponShootingEvent>,
    mut weapon_query: Query<&mut WeaponPromp>,
    mut windows: Query<&mut Window>,
    mut next_state: ResMut<NextState<WeaponState>>,
) {
    for mut window in windows.iter_mut() {
        if window.cursor.grab_mode == CursorGrabMode::Confined {
            //Center mouse becasuse confined mod is not working on Windows right now
            let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
            window.set_cursor_position(Some(center));

            for mut weapon_promp in weapon_query.iter_mut() {
                //semi auto shot
                if mouse_input.just_pressed(MouseButton::Left)
                    && weapon_promp.okay_to_shoot
                    && !weapon_promp.is_auto
                {
                    weapon_promp.mag_capacity -= 1;
                    event_writer.send(WeaponShootingEvent);
                    weapon_promp.okay_to_shoot = false;
                }
                //full auto shot
                if mouse_input.pressed(MouseButton::Left)
                    && weapon_promp.okay_to_shoot
                    && weapon_promp.is_auto
                {
                    weapon_promp.mag_capacity -= 1;
                    event_writer.send(WeaponShootingEvent);
                    weapon_promp.okay_to_shoot = false;
                }
                //reload
                if (weapon_promp.mag_capacity == 0
                    || (keyboard_input.just_pressed(KeyCode::KeyR))
                        && weapon_promp.mag_capacity < weapon_promp.self_mag_cap())
                    && weapon_promp.ammo_capacity > 0
                {
                    next_state.set(WeaponState::Reloading)
                }
            }
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

        //FIXME: lerp
        pitch_camera += 0.02;
        //yaw_camera += thread_rng().gen_range(-0.01..0.01);

        pitch_camera = pitch_camera.clamp(-PI / 2.0, PI / 2.0);
        head_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_camera)
            * Quat::from_axis_angle(Vec3::X, pitch_camera);

        persp.fov += 3.0 / 180.0 * PI;
    }

    if settings.fov < (persp.fov / PI * 180.0) {
        persp.fov -= (30.0 / 180.0 * PI) * time.delta_seconds();
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
    mut next_state: ResMut<NextState<WeaponState>>,
    time: Res<Time>,
) {
    for mut weapon_promp in weapon_query.iter_mut() {
        weapon_promp.reload_timer.tick(time.delta());

        if weapon_promp.reload_timer.finished() {
            weapon_promp.reload_timer.reset();
            weapon_promp.ammo_capacity -= weapon_promp.self_mag_cap() - weapon_promp.mag_capacity;
            weapon_promp.mag_capacity = weapon_promp.self_mag_cap();
            next_state.set(WeaponState::Shooting)
        }
    }
}

pub fn scope(
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    settings: ResMut<GameSettings>,
    mut lerp_timer: ResMut<LerpTimer>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    mut weapon_query: Query<&mut Transform, With<WeaponPromp>>,
) {
    let mut weapon_transform = weapon_query.single_mut();
    let Projection::Perspective(persp) = camera_query.single_mut().into_inner() else {
        return;
    };

    if mouse_input.just_pressed(MouseButton::Right) || mouse_input.just_released(MouseButton::Right)
    {
        lerp_timer.timer.reset()
    }
    if mouse_input.pressed(MouseButton::Right) {
        lerp_timer.timer.tick(time.delta());

        let percentage_complete =
            lerp_timer.timer.elapsed_secs() / lerp_timer.timer.duration().as_secs_f32();

        persp.fov = persp
            .fov
            .lerp((settings.fov - 40.0) / 180.0 * PI, percentage_complete);

        weapon_transform.translation = weapon_transform
            .translation
            .lerp(Vec3::new(0.0, 0.0, -0.3), percentage_complete);
    } else if !lerp_timer.timer.finished() {
        lerp_timer.timer.tick(time.delta());

        let percentage_complete =
            lerp_timer.timer.elapsed_secs() / lerp_timer.timer.duration().as_secs_f32();

        persp.fov = persp
            .fov
            .lerp(settings.fov / 180.0 * PI, percentage_complete);

        weapon_transform.translation = weapon_transform
            .translation
            .lerp(Vec3::new(0.1, -0.05, -0.2), percentage_complete);
    }
}

pub fn weapon_animation_setup(
    animations: Res<Animations>,
    mut animation_player_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut gun in &mut animation_player_query {
        gun.play(animations.0[0].clone_weak()).repeat();
        gun.set_repeat(RepeatAnimation::Count(0));
    }
}

pub fn weapon_play_animation(
    mut event_reader: EventReader<WeaponShootingEvent>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
) {
    for _event in event_reader.read() {
        for mut gun in &mut animation_player_query {
            gun.set_repeat(RepeatAnimation::Count(1));
            gun.replay();
        }
    }
}
