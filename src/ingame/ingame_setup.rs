use avian3d::prelude::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(asset_server.load("models/shooting-range.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        RigidBody::Static,
        Name::new("Shooting Range"),
    ));
    // ! Blender models looking at positive Y direction
    // ! for true mesh setup, in blender Ctrl + A -> All Transforms

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            range: 100.,
            ..default()
        },
        Transform::from_xyz(0.0, 16.0, 0.0),
        Name::new("Point Light"),
    ));

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
