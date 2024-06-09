use bevy::{animation::RepeatAnimation, prelude::*, window::CursorGrabMode};

use crate::ingame::Animations;

#[derive(Event)]
pub struct WeaponShootingEvent;

#[derive(Event)]
pub struct WeaponReloadingEvent;

#[derive(Component)]
pub struct WeaponPromp {
    pub name: String,
    pub mag_capacity: u8,
    pub ammo_capacity: u8,
    pub head_damage: u8,
    pub body_damage: u8,
    pub is_auto: bool,
    pub okay_to_shoot: bool,
    pub is_reloading: bool,
    pub firerate: Timer,
    pub reload: Timer,
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
            is_reloading: false,
            firerate: Timer::from_seconds(0.1, TimerMode::Once),
            reload: Timer::from_seconds(1.0, TimerMode::Once),
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
            is_reloading: false,
            firerate: Timer::from_seconds(0.08, TimerMode::Once),
            reload: Timer::from_seconds(2.0, TimerMode::Once),
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
            is_reloading: false,
            firerate: Timer::from_seconds(1.5, TimerMode::Once),
            reload: Timer::from_seconds(2.5, TimerMode::Once),
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

    pub fn self_firerate(&self) -> Timer {
        match self.name.as_str() {
            "P226" => WeaponPromp::p226().firerate,
            "AK-15" => WeaponPromp::ak15().firerate,
            "MSR" => WeaponPromp::msr().firerate,
            _ => panic!("No gun found for self_firerate"),
        }
    }

    pub fn self_reload(&self) -> Timer {
        match self.name.as_str() {
            "P226" => WeaponPromp::p226().reload,
            "AK-15" => WeaponPromp::ak15().reload,
            "MSR" => WeaponPromp::msr().reload,
            _ => panic!("No gun found for self_reload"),
        }
    }
}

pub fn shooting_event(
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<WeaponShootingEvent>,
    mut weapon_query: Query<&mut WeaponPromp>,
    mut windows: Query<&mut Window>,
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
                    && !weapon_promp.is_reloading
                {
                    weapon_promp.mag_capacity -= 1;
                    event_writer.send(WeaponShootingEvent);
                    weapon_promp.okay_to_shoot = false;
                }
                //full auto shot
                if mouse_input.pressed(MouseButton::Left)
                    && weapon_promp.okay_to_shoot
                    && weapon_promp.is_auto
                    && !weapon_promp.is_reloading
                {
                    weapon_promp.mag_capacity -= 1;
                    event_writer.send(WeaponShootingEvent);
                    weapon_promp.okay_to_shoot = false;
                }
                //reload
                if (weapon_promp.mag_capacity == 0 || keyboard_input.just_pressed(KeyCode::KeyR))
                    && weapon_promp.mag_capacity < weapon_promp.self_mag_cap()
                {
                    weapon_promp.is_reloading = true;
                }
            }
        }
    }
}

pub fn firerate_timer(mut weapon_query: Query<&mut WeaponPromp>, time: Res<Time>) {
    for mut weapon_promp in weapon_query.iter_mut() {
        if !weapon_promp.okay_to_shoot && !weapon_promp.is_reloading {
            weapon_promp.firerate.tick(time.delta());

            if weapon_promp.firerate.finished() {
                weapon_promp.okay_to_shoot = true;
                weapon_promp.firerate = weapon_promp.self_firerate();
            }
        }
    }
}

pub fn reload_timer(mut weapon_query: Query<&mut WeaponPromp>, time: Res<Time>) {
    for mut weapon_promp in weapon_query.iter_mut() {
        if weapon_promp.is_reloading {
            weapon_promp.reload.tick(time.delta());

            if weapon_promp.reload.finished() {
                weapon_promp.is_reloading = false;
                weapon_promp.ammo_capacity -=
                    weapon_promp.self_mag_cap() - weapon_promp.mag_capacity;
                weapon_promp.reload = weapon_promp.self_reload();
                weapon_promp.mag_capacity = weapon_promp.self_mag_cap();
            }
        }
    }
}

pub fn weapon_animation_setup(
    animations: Res<Animations>,
    mut animation_player_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut gun in &mut animation_player_query {
        gun.play(animations.0[1].clone_weak()).repeat();
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
