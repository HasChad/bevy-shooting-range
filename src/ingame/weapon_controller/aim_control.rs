#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use std::f32::consts::PI;

use super::{weapons::WeaponActionState, WeaponPromp};
use crate::ingame::{
    crosshair::{CrosshairLine, CrosshairLineSettings},
    GameSettings, KeyBindings,
};

#[derive(Resource)]
pub struct LerpTimer {
    scope_timer: Timer,
}

impl Default for LerpTimer {
    fn default() -> Self {
        LerpTimer {
            scope_timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

pub fn scope(
    time: Res<Time>,
    settings: ResMut<GameSettings>,
    key_bindings: Res<KeyBindings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    weapon_action_state: Res<State<WeaponActionState>>,
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

    if mouse_input.just_pressed(key_bindings.scope)
        || mouse_input.just_released(key_bindings.scope)
        || weapon_action_state.is_changed()
    {
        lerp_timer.scope_timer.reset()
    }
    if mouse_input.pressed(key_bindings.scope)
        && *weapon_action_state.get() == WeaponActionState::Shooting
    {
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
