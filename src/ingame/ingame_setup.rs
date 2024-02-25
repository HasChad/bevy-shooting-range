use bevy::prelude::*;
use bevy_xpbd_3d::{math::PI, prelude::*};

use crate::ingame::Animations;

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.8, 0.0),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 90.0 / 180.0 * PI, // ! One PI = 180, first value is the real fov
                    near: 0.01,
                    ..default()
                }),

                ..default()
            },
            FogSettings {
                color: Color::BLACK,
                falloff: FogFalloff::Exponential { density: 1.0 },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("models/p226_anim.glb#Scene0"),
                    transform: Transform::from_xyz(0.15, -0.1, -0.19),
                    ..default()
                },
                Name::new("P226"),
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                Collider::sphere(0.01),
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, -1.0)),
                Name::new("BulletSpawner"),
            ));
        });
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //gun animation load
    commands.insert_resource(Animations(vec![
        asset_server.load("models/p226_anim.glb#Animation0")
    ]));

    //crosshair test
    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Px(2.0),
            width: Val::Px(6.0),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    });
    commands.spawn(NodeBundle {
        style: Style {
            height: Val::Px(6.0),
            width: Val::Px(2.0),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        background_color: Color::WHITE.into(),
        ..default()
    });

    //shooting range model
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/shooting-range.glb#Scene0"),
            ..default()
        },
        Name::new("Shooting Range"),
    ));
    // ! Blender models looking at positive Y direction

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
