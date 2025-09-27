use avian3d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = false;

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
            shadows_enabled: true,
            range: 100.,
            ..default()
        },
        Transform::from_xyz(0.0, 16.0, 0.0),
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.7, 0.7, 0.7),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.02, -0.05, -0.02), Vec3::Y),
        Name::new("Sun"),
    ));
}
