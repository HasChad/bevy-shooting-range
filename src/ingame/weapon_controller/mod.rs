use bevy::prelude::*;

pub mod bullet;
pub mod weapon_control;
pub mod weapons;

use bullet::*;
use weapon_control::*;
use weapons::*;

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
                //weapon_control systems
                (shooting_event, firerate_timer).run_if(in_state(WeaponActionState::Shooting)),
                reload_timer.run_if(in_state(WeaponActionState::Reloading)),
                scope,
                shooting_sound,
                weapon_animation_setup,
                weapon_play_animation,
                shooting_camera_shake,
                change_weapon,
                sway_weapon,
                //bullet systems
                spawn_bullet,
            ),
        )
        .add_systems(FixedUpdate, bullet_controller)
        //events
        //plugins
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
