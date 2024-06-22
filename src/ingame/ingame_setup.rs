use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Resource)]
pub struct ShootingAnimations(pub Vec<Handle<AnimationClip>>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //gun animation load
    commands.insert_resource(ShootingAnimations(vec![
        //shooting animations
        asset_server.load("models/weapons/P226.glb#Animation0"),
        asset_server.load("models/weapons/AK15.glb#Animation0"),
        asset_server.load("models/weapons/FNFAL.glb#Animation0"),
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
