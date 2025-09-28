use avian3d::prelude::{Physics, PhysicsTime};
use bevy::{prelude::*, window::CursorGrabMode};

pub mod ingame_setup;
pub mod ingame_ui;
pub mod player_controller;
pub mod target_controller;
pub mod weapon_controller;

use ingame_setup::*;
use ingame_ui::*;
use player_controller::*;
use target_controller::*;
use weapon_controller::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum PlayableState {
    #[default]
    Menu,
    Action,
}

#[derive(Resource)]
pub struct GameSettings {
    pub sensitivity: f32,
    pub fov: f32,
    pub volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            sensitivity: 1.0,
            fov: 90.0,
            volume: 0.5,
        }
    }
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, edit_mode_toggler)
            //resources
            .init_resource::<GameSettings>()
            //states
            .init_state::<PlayableState>()
            //plugins
            .add_plugins(IngameUIPlugin)
            .add_plugins(PlayerControllerPlugin)
            .add_plugins(TargetControllerPlugin)
            .add_plugins(WeaponControllerPlugin);
    }
}

pub fn edit_mode_toggler(
    mut time: ResMut<Time<Physics>>,
    key_bindings: Res<KeyBindings>,
    mut window: Single<&mut Window>,
    input: ResMut<ButtonInput<KeyCode>>,
    state: Res<State<PlayableState>>,
    mut next_state: ResMut<NextState<PlayableState>>,
) {
    if input.just_pressed(key_bindings.focus) {
        match state.get() {
            PlayableState::Action => {
                next_state.set(PlayableState::Menu);
                time.pause();
                window.cursor_options.visible = true;
                window.cursor_options.grab_mode = CursorGrabMode::None;
            }
            PlayableState::Menu => {
                next_state.set(PlayableState::Action);
                time.unpause();
                window.cursor_options.visible = false;
                window.cursor_options.grab_mode = CursorGrabMode::Locked;
            }
        }
    }

    if *state.get() == PlayableState::Action {
        let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        window.set_cursor_position(Some(center));
    }
}
