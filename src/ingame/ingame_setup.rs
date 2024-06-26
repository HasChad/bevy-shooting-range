use bevy::{prelude::*, window::CursorGrabMode};
use bevy_xpbd_3d::prelude::*;

#[derive(Resource)]
pub struct ShootingAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct ReloadingAnimations(pub Vec<Handle<AnimationClip>>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ShootingAnimations(vec![
        asset_server.load("models/weapons/P226.glb#Animation0"),
        asset_server.load("models/weapons/AK15.glb#Animation0"),
        asset_server.load("models/weapons/FNFAL.glb#Animation0"),
        asset_server.load("models/weapons/P226.glb#Animation0"),
    ]));

    commands.insert_resource(ReloadingAnimations(vec![
        asset_server.load("models/weapons/P226.glb#Animation1"),
        asset_server.load("models/weapons/AK15.glb#Animation1"),
        asset_server.load("models/weapons/FNFAL.glb#Animation1"),
        asset_server.load("models/weapons/P226.glb#Animation1"),
    ]));

    //shooting range
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/shooting-range.glb#Scene0"),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        RigidBody::Static,
        Name::new("Shooting Range"),
    ));
    // ! Blender models looking at positive Y direction
    // ! for true mesh setup, in blender Ctrl + A -> All Transforms

    //point light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10_000_000.,
            range: 100.,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 0.0),
        ..default()
    });
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

    if window.cursor.grab_mode == CursorGrabMode::Confined {
        //Center mouse becasuse confined mod is not working on Windows right now
        let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        window.set_cursor_position(Some(center));
    }

    if input.just_pressed(KeyCode::Escape) {
        match window.cursor.grab_mode {
            CursorGrabMode::Confined => {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
                next_state.set(PlayableState::NoAction)
            }
            _ => {
                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
                next_state.set(PlayableState::Action)
            }
        }
    }
}
