use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum WeaponActionState {
    #[default]
    Shooting,
    Reloading,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum WeaponState {
    #[default]
    P226,
    AK15,
    FNFAL,
    MSR,
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
            firerate: Timer::from_seconds(0.09, TimerMode::Once),
            reload_timer: Timer::from_seconds(2.0, TimerMode::Once),
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
