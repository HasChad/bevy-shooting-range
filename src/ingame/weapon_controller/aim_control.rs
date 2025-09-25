use bevy::prelude::*;
use std::f32::consts::PI;

use super::{
    weapons::{WeaponActionState, WeaponAimState},
    Weapon,
};
use crate::ingame::{
    crosshair::{CrosshairLine, CrosshairLineSettings},
    GameSettings, KeyBindings,
};

pub fn aim_changer(
    key_bindings: Res<KeyBindings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    weapon_action_state: Res<State<WeaponActionState>>,
    mut next_weapon_aim_state: ResMut<NextState<WeaponAimState>>,
) {
    if mouse_input.pressed(key_bindings.scope)
        && (*weapon_action_state.get() == WeaponActionState::Shoot
            || *weapon_action_state.get() == WeaponActionState::Ready)
    {
        next_weapon_aim_state.set(WeaponAimState::Scope);
    } else {
        next_weapon_aim_state.set(WeaponAimState::HipFire);
    }
}

pub fn scope(
    time: Res<Time>,
    settings: ResMut<GameSettings>,
    crosshair_settings: Res<CrosshairLineSettings>,
    weapon_aim_state: Res<State<WeaponAimState>>,
    mut camera_projection: Single<&mut Projection, With<Camera3d>>,
    mut weapon_transform: Single<&mut Transform, With<Weapon>>,
    mut crosshair_query: Query<&mut Visibility, With<CrosshairLine>>,
) {
    let Projection::Perspective(persp) = camera_projection.as_mut() else {
        return;
    };

    if *weapon_aim_state.get() == WeaponAimState::Scope {
        for mut cross_visib in crosshair_query.iter_mut() {
            *cross_visib = Visibility::Hidden;
        }
        weapon_transform.translation.smooth_nudge(
            &Vec3::new(0.0, 0.0, -0.15),
            20.0,
            time.delta_secs(),
        );
        persp
            .fov
            .smooth_nudge(&(50.0 / 180.0 * PI), 20.0, time.delta_secs());
    } else {
        for mut cross_visib in crosshair_query.iter_mut() {
            *cross_visib = crosshair_settings.enable;
        }
        weapon_transform.translation.smooth_nudge(
            &Vec3::new(0.075, -0.04, -0.1),
            20.0,
            time.delta_secs(),
        );
        persp
            .fov
            .smooth_nudge(&(settings.fov / 180.0 * PI), 20.0, time.delta_secs());
    }
}
