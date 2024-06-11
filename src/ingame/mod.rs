use bevy::prelude::*;

pub mod bullet;
pub mod gun;
pub mod ingame_setup;
pub mod ingame_ui;
pub mod player_controller;
pub mod targets;

use bullet::*;
use gun::*;
use ingame_setup::*;
use ingame_ui::*;
use player_controller::*;
use targets::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

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
                    (shooting_event, scope, firerate_timer).run_if(in_state(WeaponState::Shooting)),
                    reload_timer.run_if(in_state(WeaponState::Reloading)),
                    weapon_animation_setup,
                    weapon_play_animation,
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
            //events
            .add_event::<WeaponShootingEvent>()
            .add_event::<WeaponReloadingEvent>()
            .add_event::<HitConfirmEvent>()
            //states
            .init_state::<WeaponState>();
    }
}
