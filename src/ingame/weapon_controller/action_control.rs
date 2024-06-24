#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;

use super::{WeaponActionState, WeaponPromp, WeaponReloadingEvent, WeaponShootingEvent};
use crate::ingame::KeyBindings;

pub fn shooting_event(
    key_bindings: Res<KeyBindings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut shot_event_writer: EventWriter<WeaponShootingEvent>,
    mut reload_event_writer: EventWriter<WeaponReloadingEvent>,
    mut weapon_query: Query<&mut WeaponPromp>,
    mut next_state: ResMut<NextState<WeaponActionState>>,
) {
    for mut weapon_promp in weapon_query.iter_mut() {
        if weapon_promp.okay_to_shoot {
            //semi auto shot
            if mouse_input.just_pressed(key_bindings.fire) && !weapon_promp.is_auto {
                weapon_promp.mag_capacity -= 1;
                shot_event_writer.send(WeaponShootingEvent);
                weapon_promp.okay_to_shoot = false;
            }
            //full auto shot
            if mouse_input.pressed(key_bindings.fire) && weapon_promp.is_auto {
                weapon_promp.mag_capacity -= 1;
                shot_event_writer.send(WeaponShootingEvent);
                weapon_promp.okay_to_shoot = false;
            }
        }
        //reload
        if (weapon_promp.mag_capacity == 0
            || (keyboard_input.just_pressed(key_bindings.reload))
                && weapon_promp.mag_capacity < weapon_promp.self_mag_cap())
            && weapon_promp.ammo_capacity > 0
        {
            reload_event_writer.send(WeaponReloadingEvent);
            next_state.set(WeaponActionState::Reloading);
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
