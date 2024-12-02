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
    mut windows: Query<&mut Window>,
    mut next_state: ResMut<NextState<PlayableState>>,
) {
    let mut window = windows.single_mut();

    if window.cursor_options.grab_mode == CursorGrabMode::Confined {
        //Center mouse becasuse confined mod is not working on Windows right now
        let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        window.set_cursor_position(Some(center));
    }

    if input.just_pressed(KeyCode::Escape) {
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
