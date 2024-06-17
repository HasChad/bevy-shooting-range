use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use super::Animations;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    /*
    //testing purpose only 4 now
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    */
) {
    /*
    //cube spawn
    let mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));
    commands.spawn((
        RigidBody::Dynamic,
        Collider::trimesh_from_mesh(&mesh).unwrap(),
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb_u8(0, 144, 0)),
            transform: Transform::from_xyz(0.0, 0.5, -1.0),
            ..default()
        },
    ));
    */

    /*
        //Png Scope
        commands.spawn(ImageBundle {
            image: UiImage::new(asset_server.load("png-scope.png")),
            transform: Transform::from_scale(Vec3::new(0.45, 0.45, 1.0)),
            style: Style {
                position_type: PositionType::Absolute,
                min_width: Val::Px(100.),
                min_height: Val::Px(100.),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    right: Val::Px(62.0),
                    top: Val::Px(123.0),
                    ..default()
                },
                ..default()
            },
            background_color: Color::WHITE.into(),

            ..Default::default()
        });
    */

    //gun animation load
    commands.insert_resource(Animations(vec![
        //shooting animations
        asset_server.load("models/weapons/P226.glb#Animation1"),
        asset_server.load("models/weapons/AK15.glb#Animation0"),
        asset_server.load("models/weapons/FNFAL.glb#Animation0"),
    ]));

    //shooting range model
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
