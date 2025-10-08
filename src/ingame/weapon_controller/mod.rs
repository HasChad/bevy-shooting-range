use bevy::prelude::*;

pub mod action_control;
pub mod aim_control;
pub mod bullet;
pub mod weapon_animations;
pub mod weapon_control;
pub mod weapons;

use action_control::*;
use aim_control::*;
use bullet::*;
use weapon_animations::*;
use weapon_control::*;
use weapons::*;

use super::PlayableState;

#[derive(Message)]
pub struct WeaponShootingMessage;

#[derive(Message)]
pub struct WeaponReloadingMessage;

#[derive(Message)]
pub struct HitConfirmMessage {
    pub hit_entity: Entity,
    pub hit_normal: Vec3,
}

pub struct WeaponControllerPlugin;

impl Plugin for WeaponControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                // animations
                p226_animation_setup,
                ak15_animation_setup,
            ),
        )
        .add_systems(
            Update,
            (
                // animation system
                weapon_animation_setup,
                weapon_animation_player,
                // aim system
                aim_changer,
                scope,
                // control system
                camera_recoil,
                (
                    sway_weapon.run_if(in_state(WeaponAimState::HipFire)),
                    scoped_sway_weapon.run_if(in_state(WeaponAimState::Scope)),
                ),
                //weapon_sounds,
                spawn_bullet,
                // action system
                firerate_timer.run_if(in_state(WeaponActionState::Shoot)),
                reload_timer.run_if(in_state(WeaponActionState::Reload)),
                (
                    change_weapon,
                    weapon_input_controller.run_if(in_state(WeaponActionState::Ready)),
                ),
            )
                .run_if(in_state(PlayableState::Action)),
        )
        .add_systems(
            FixedUpdate,
            bullet_controller.run_if(in_state(PlayableState::Action)),
        )
        // resources
        .init_resource::<WeaponRes>()
        // states
        .init_state::<WeaponActionState>()
        .init_state::<WeaponAimState>()
        .init_state::<WeaponState>()
        // messages
        .add_message::<WeaponShootingMessage>()
        .add_message::<WeaponReloadingMessage>()
        .add_message::<HitConfirmMessage>();
    }
}
