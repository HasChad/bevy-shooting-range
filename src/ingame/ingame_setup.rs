use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use std::f32::consts::PI;

use crate::ingame::Animations;

#[derive(Component)]
pub struct P226 {
    pub lifetime: Timer,
    pub okay_to_shoot: bool,
}

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
        //gun model
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("models/p226_anim.glb#Scene0"),
                    transform: Transform::from_xyz(0.15, -0.1, -0.19),
                    ..default()
                },
                P226 {
                    lifetime: Timer::from_seconds(0.2, TimerMode::Once),
                    okay_to_shoot: true,
                },
                Name::new("P226"),
            ));
        })
        //RayCast
        .with_children(|parent| {
            parent.spawn((
                RayCaster::new(Vec3::ZERO, Direction3d::NEG_Z),
                Name::new("RayCast"),
            ));
        });
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //gun animation load
    commands.insert_resource(Animations(vec![
        asset_server.load("models/p226_anim.glb#Animation0"),
        asset_server.load("models/p226_anim.glb#Animation1"),
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
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
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
