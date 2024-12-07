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

#[derive(Component, Clone)]
pub struct WeaponPromp {
    pub name: String,
    pub mag_capacity: u8,
    pub ammo_capacity: u8,
    pub head_damage: u8,
    pub body_damage: u8,
    pub is_auto: bool,
    pub firerate: Timer,
    pub reload_timer: Timer,
    //pub time_to_aim: Timer,
}

#[derive(Resource, Clone)]
pub struct WeaponRes {
    pub p226: WeaponPromp,
    pub ak15: WeaponPromp,
}

impl Default for WeaponRes {
    fn default() -> Self {
        Self {
            p226: WeaponPromp::p226(),
            ak15: WeaponPromp::ak15(),
        }
    }
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
            firerate: Timer::from_seconds(0.09, TimerMode::Once),
            reload_timer: Timer::from_seconds(1.75, TimerMode::Once),
        }
    }

    pub fn self_mag_cap(&self) -> u8 {
        match self.name.as_str() {
            "P226" => WeaponPromp::p226().mag_capacity,
            "AK-15" => WeaponPromp::ak15().mag_capacity,
            _ => panic!("No gun found for self_mag_cap"),
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
    mut weapon_query: Query<(&mut WeaponPromp, &mut SceneRoot)>,
) {
    for key in input.get_just_pressed() {
        if *weapon_action_state.get() == WeaponActionState::Ready
            && *weapon_aim_state.get() == WeaponAimState::HipFire
        {
            for (mut weapon_promp, mut weapon_scene) in weapon_query.iter_mut() {
                match weapon_state.get() {
                    WeaponState::P226 => weapon_res.p226 = weapon_promp.clone(),
                    WeaponState::AK15 => weapon_res.ak15 = weapon_promp.clone(),
                }
                let key = *key;
                match key {
                    KeyCode::Digit1 => {
                        *weapon_promp = weapon_res.p226.clone();
                        *weapon_scene =
                            SceneRoot(asset_server.load("models/weapons/P226.glb#Scene0"));
                        next_weapon_state.set(WeaponState::P226);
                        //next_weapon_action_state.set(WeaponActionState::Raise);
                    }
                    KeyCode::Digit2 => {
                        *weapon_promp = weapon_res.ak15.clone();
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
