use bevy::prelude::*;

pub mod bullet;
pub mod ingame_setup;
pub mod ingame_ui;
pub mod player_controller;
pub mod targets;
pub mod weapon_control;

use bullet::*;
use ingame_setup::*;
use ingame_ui::*;
use player_controller::*;
use targets::*;
use weapon_control::*;

#[derive(Event)]
pub struct WeaponShootingEvent;

#[derive(Event)]
pub struct WeaponReloadingEvent;

#[derive(Event)]
pub struct HitConfirmEvent {
    pub hit_entity: Entity,
    pub hit_normal: Vec3,
}

#[derive(Resource)]
pub struct ShootingAnimations(Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct GameSettings {
    pub sensitivity: f32,
    pub player_speed: f32,
    pub fov: f32,
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, target_setup))
            .add_systems(
                Update,
                (
                    //gun systems
                    (shooting_event, firerate_timer).run_if(in_state(WeaponActionState::Shooting)),
                    reload_timer.run_if(in_state(WeaponActionState::Reloading)),
                    scope,
                    shooting_sound,
                    weapon_animation_setup,
                    weapon_play_animation,
                    shooting_camera_shake,
                    //bullet systems
                    spawn_bullet,
                    //target systems
                    circle_target_controller,
                    enemy_target_controller,
                    enemy_target_hostage_controller,
                ),
            )
            .add_systems(FixedUpdate, bullet_controller)
            //plugins
            .add_plugins(PlayerControllerPlugin)
            .add_plugins(IngameUIPlugin)
            //resources
            .insert_resource(GameSettings {
                sensitivity: 1.0,
                player_speed: 5.0,
                fov: 90.0,
            })
            .init_resource::<LerpTimer>()
            .init_resource::<HitCounters>()
            //events
            .add_event::<WeaponShootingEvent>()
            .add_event::<WeaponReloadingEvent>()
            .add_event::<HitConfirmEvent>()
            //states
            .init_state::<WeaponActionState>()
            .init_state::<WeaponState>();
    }
}
