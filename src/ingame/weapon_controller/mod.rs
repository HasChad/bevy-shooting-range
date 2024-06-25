use bevy::prelude::*;

pub mod action_control;
pub mod aim_control;
pub mod bullet;
pub mod weapon_control;
pub mod weapons;

use action_control::*;
use aim_control::*;
use bullet::*;
use weapon_control::*;
use weapons::*;

use super::PlayableState;

#[derive(Event)]
pub struct WeaponShootingEvent;

#[derive(Event)]
pub struct WeaponReloadingEvent;

#[derive(Event)]
pub struct HitConfirmEvent {
    pub hit_entity: Entity,
    pub hit_normal: Vec3,
}

pub struct WeaponControllerPlugin;

impl Plugin for WeaponControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sway_weapon,
                shooting_sound,
                scope,
                weapon_play_animation,
                firerate_timer.run_if(in_state(WeaponActionState::Shooting)),
                reload_timer.run_if(in_state(WeaponActionState::Reloading)),
                (
                    shooting_event.run_if(in_state(WeaponActionState::Shooting)),
                    //weapon control
                    camera_recoil,
                    change_weapon,
                    //bullet systems
                    spawn_bullet,
                )
                    .run_if(in_state(PlayableState::Action)),
            ),
        )
        .add_systems(FixedUpdate, bullet_controller)
        //resources
        .init_resource::<LerpTimer>()
        .init_resource::<WeaponRes>()
        //states
        .init_state::<WeaponActionState>()
        .init_state::<WeaponAimState>()
        .init_state::<WeaponState>()
        //events
        .add_event::<WeaponShootingEvent>()
        .add_event::<WeaponReloadingEvent>()
        .add_event::<HitConfirmEvent>();
    }
}
