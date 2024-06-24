use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum WeaponActionState {
    #[default]
    Shooting,
    Reloading,
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
    FNFAL,
    MSR,
}

#[derive(Component, Clone)]
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

#[derive(Resource, Clone)]
pub struct WeaponRes {
    p226: WeaponPromp,
    ak15: WeaponPromp,
    fnfal: WeaponPromp,
    msr: WeaponPromp,
}

impl Default for WeaponRes {
    fn default() -> Self {
        Self {
            p226: WeaponPromp::p226(),
            ak15: WeaponPromp::ak15(),
            fnfal: WeaponPromp::fn_fal(),
            msr: WeaponPromp::msr(),
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
            firerate: Timer::from_seconds(0.09, TimerMode::Once),
            reload_timer: Timer::from_seconds(1.75, TimerMode::Once),
        }
    }

    pub fn fn_fal() -> WeaponPromp {
        WeaponPromp {
            name: "FN-FAL".to_owned(),
            mag_capacity: 20,
            ammo_capacity: 80,
            head_damage: 15,
            body_damage: 8,
            is_auto: false,
            okay_to_shoot: true,
            firerate: Timer::from_seconds(0.12, TimerMode::Once),
            reload_timer: Timer::from_seconds(2.2, TimerMode::Once),
        }
    }

    pub fn msr() -> WeaponPromp {
        WeaponPromp {
            name: "MSR".to_owned(),
            mag_capacity: 5,
            ammo_capacity: 20,
            head_damage: 20,
            body_damage: 12,
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
            "FN-FAL" => WeaponPromp::fn_fal().mag_capacity,
            "MSR" => WeaponPromp::msr().mag_capacity,
            _ => panic!("No gun found for self_mag_cap"),
        }
    }
}

pub fn change_weapon(
    mut weapon_query: Query<(&mut WeaponPromp, &mut Handle<Scene>)>,
    input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut next_weapon_state: ResMut<NextState<WeaponState>>,
    weapon_state: Res<State<WeaponState>>,
    weapon_action_state: Res<State<WeaponActionState>>,
    mut weapon_res: ResMut<WeaponRes>,
) {
    for key in input.get_just_pressed() {
        if *weapon_action_state.get() == WeaponActionState::Shooting {
            for (mut weapon_promp, mut weapon_scene) in weapon_query.iter_mut() {
                match weapon_state.get() {
                    WeaponState::P226 => weapon_res.p226 = weapon_promp.clone(),
                    WeaponState::AK15 => weapon_res.ak15 = weapon_promp.clone(),
                    WeaponState::FNFAL => weapon_res.fnfal = weapon_promp.clone(),
                    WeaponState::MSR => weapon_res.msr = weapon_promp.clone(),
                }
                let key = *key;
                match key {
                    KeyCode::Digit1 => {
                        *weapon_promp = weapon_res.p226.clone();
                        *weapon_scene = asset_server.load("models/weapons/P226.glb#Scene0");
                        next_weapon_state.set(WeaponState::P226);
                    }
                    KeyCode::Digit2 => {
                        *weapon_promp = weapon_res.ak15.clone();
                        *weapon_scene = asset_server.load("models/weapons/AK15.glb#Scene0");
                        next_weapon_state.set(WeaponState::AK15);
                    }
                    KeyCode::Digit3 => {
                        *weapon_promp = weapon_res.fnfal.clone();
                        *weapon_scene = asset_server.load("models/weapons/FNFAL.glb#Scene0");
                        next_weapon_state.set(WeaponState::FNFAL);
                    }
                    KeyCode::Digit4 => {
                        *weapon_promp = weapon_res.msr.clone();
                        *weapon_scene = asset_server.load("models/weapons/MSR.glb#Scene0");
                        next_weapon_state.set(WeaponState::MSR);
                    }
                    _ => (),
                }
            }
        }
    }
}
