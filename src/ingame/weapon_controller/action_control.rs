#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

use super::{Weapon, WeaponActionState, WeaponReloadingMessage, WeaponShootingMessage};
use crate::ingame::KeyBindings;

pub fn weapon_input_controller(
    key_bindings: Res<KeyBindings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut weapon: Single<&mut Weapon>,
    mut next_weapon_action_state: ResMut<NextState<WeaponActionState>>,
    mut shot_mes_writer: MessageWriter<WeaponShootingMessage>,
    mut reload_mes_writer: MessageWriter<WeaponReloadingMessage>,
) {
    //shoot
    if ((mouse_input.just_pressed(key_bindings.fire) && !weapon.is_auto)
        || (mouse_input.pressed(key_bindings.fire) && weapon.is_auto))
        && weapon.mag_count > 0
    {
        weapon.mag_count -= 1;
        shot_mes_writer.write(WeaponShootingMessage);
        next_weapon_action_state.set(WeaponActionState::Shoot);
    }

    //reload
    if (weapon.mag_count == 0 || keyboard_input.just_pressed(key_bindings.reload))
        && weapon.ammo_count > 0
        && weapon.mag_count < weapon.mag_capacity
    {
        reload_mes_writer.write(WeaponReloadingMessage);
        next_weapon_action_state.set(WeaponActionState::Reload);
    }
}

pub fn firerate_timer(
    time: Res<Time>,
    mut weapon: Single<&mut Weapon>,
    mut next_state: ResMut<NextState<WeaponActionState>>,
) {
    weapon.firerate.tick(time.delta());

    if weapon.firerate.is_finished() {
        weapon.firerate.reset();
        next_state.set(WeaponActionState::Ready);
    }
}

pub fn reload_timer(
    time: Res<Time>,
    mut weapon: Single<&mut Weapon>,
    mut next_state: ResMut<NextState<WeaponActionState>>,
) {
    weapon.reload_timer.tick(time.delta());

    if weapon.reload_timer.is_finished() {
        weapon.reload_timer.reset();

        if weapon.ammo_count + weapon.mag_count >= weapon.mag_capacity {
            weapon.ammo_count -= weapon.mag_capacity - weapon.mag_count;
            weapon.mag_count = weapon.mag_capacity;
        } else {
            weapon.mag_count += weapon.ammo_count;
            weapon.ammo_count = 0;
        }

        next_state.set(WeaponActionState::Ready)
    }
}
