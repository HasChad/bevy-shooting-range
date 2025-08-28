use avian3d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //shooting range
    commands.spawn((
        SceneRoot(asset_server.load("models/shooting-range.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
        Name::new("Shooting Range"),
    ));
    // ! Blender models looking at positive Y direction
    // ! for true mesh setup, in blender Ctrl + A -> All Transforms

    //point light
    commands.spawn((
        PointLight {
            intensity: 10_000_000.,
            range: 100.,
            ..default()
        },
        Transform::from_xyz(0.0, 16.0, 0.0),
    ));
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum PlayableState {
    #[default]
    NoAction,
    Action,
}

pub fn edit_mode_toggler(
    input: ResMut<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    mut next_state: ResMut<NextState<PlayableState>>,
) {
    if input.just_pressed(KeyCode::Tab) {
        match window.cursor_options.grab_mode {
            CursorGrabMode::Confined => {
                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;
                next_state.set(PlayableState::NoAction)
            }
            _ => {
                window.cursor_options.grab_mode = CursorGrabMode::Confined;
                window.cursor_options.visible = false;
                next_state.set(PlayableState::Action)
            }
        }
    }
}
