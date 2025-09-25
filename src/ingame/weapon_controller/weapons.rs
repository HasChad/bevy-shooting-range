#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum WeaponActionState {
    #[default]
    Ready,
    Shoot,
    Reload,
    Raise,
    Lower,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum WeaponAimState {
    #[default]
    HipFire,
    Scope,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum WeaponState {
    #[default]
    P226,
    AK15,
}

#[derive(Resource, Clone)]
pub struct WeaponRes {
    pub p226: Weapon,
    pub ak15: Weapon,
}

impl Default for WeaponRes {
    fn default() -> Self {
        Self {
            p226: Weapon::p226(),
            ak15: Weapon::ak15(),
        }
    }
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: String,
    pub mag_count: u8,
    pub ammo_count: u8,
    pub mag_capacity: u8,
    pub ammo_capacity: u8,
    pub head_damage: u8,
    pub body_damage: u8,
    pub is_auto: bool,
    pub firerate: Timer,
    pub reload_timer: Timer,
    pub raise_timer: Timer,
    pub lower_timer: Timer,
    pub time_to_aim: f32,
}

impl Weapon {
    pub fn p226() -> Weapon {
        Weapon {
            name: "P226".to_owned(),
            mag_count: 15,
            ammo_count: 60,
            mag_capacity: 15,
            ammo_capacity: 60,
            head_damage: 3,
            body_damage: 1,
            is_auto: false,
            firerate: Timer::from_seconds(0.1, TimerMode::Once),
            reload_timer: Timer::from_seconds(1.0, TimerMode::Once),
            raise_timer: Timer::from_seconds(1.0, TimerMode::Once),
            lower_timer: Timer::from_seconds(1.0, TimerMode::Once),
            time_to_aim: 20.0,
        }
    }

    pub fn ak15() -> Weapon {
        Weapon {
            name: "AK-15".to_owned(),
            mag_count: 30,
            ammo_count: 120,
            mag_capacity: 30,
            ammo_capacity: 120,
            head_damage: 10,
            body_damage: 4,
            is_auto: true,
            firerate: Timer::from_seconds(0.09, TimerMode::Once),
            reload_timer: Timer::from_seconds(1.75, TimerMode::Once),
            raise_timer: Timer::from_seconds(1.0, TimerMode::Once),
            lower_timer: Timer::from_seconds(1.0, TimerMode::Once),
            time_to_aim: 20.0,
        }
    }
}

pub fn change_weapon(
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut weapon_res: ResMut<WeaponRes>,
    weapon_state: Res<State<WeaponState>>,
    weapon_aim_state: Res<State<WeaponAimState>>,
    weapon_action_state: Res<State<WeaponActionState>>,
    //mut next_weapon_action_state: ResMut<NextState<WeaponActionState>>,
    mut next_weapon_state: ResMut<NextState<WeaponState>>,
    mut weapon_query: Query<(&mut Weapon, &mut SceneRoot)>,
) {
    for key in input.get_just_pressed() {
        if *weapon_action_state.get() == WeaponActionState::Ready
            && *weapon_aim_state.get() == WeaponAimState::HipFire
        {
            for (mut weapon, mut weapon_scene) in weapon_query.iter_mut() {
                match weapon_state.get() {
                    WeaponState::P226 => weapon_res.p226 = weapon.clone(),
                    WeaponState::AK15 => weapon_res.ak15 = weapon.clone(),
                }
                let key = *key;
                match key {
                    KeyCode::Digit1 => {
                        *weapon = weapon_res.p226.clone();
                        *weapon_scene =
                            SceneRoot(asset_server.load("models/weapons/P226.glb#Scene0"));
                        next_weapon_state.set(WeaponState::P226);
                        //next_weapon_action_state.set(WeaponActionState::Raise);
                    }
                    KeyCode::Digit2 => {
                        *weapon = weapon_res.ak15.clone();
                        *weapon_scene =
                            SceneRoot(asset_server.load("models/weapons/AK15.glb#Scene0"));
                        next_weapon_state.set(WeaponState::AK15);
                        //next_weapon_action_state.set(WeaponActionState::Raise);
                    }
                    _ => (),
                }
            }
        }
    }
}
