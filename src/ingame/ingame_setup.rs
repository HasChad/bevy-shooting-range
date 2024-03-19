use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use std::f32::consts::PI;

use super::Animations;
use super::GameSettings;

#[derive(Component)]
pub struct P226 {
    pub lifetime: Timer,
    pub okay_to_shoot: bool,
}

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    //Player
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.8, 0.0),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: settings.fov / 180.0 * PI, // ! One PI = 180, first value is the real fov
                    near: 0.01,
                    ..default()
                }),

                ..default()
            },
            FogSettings {
                color: Color::BLACK,
                falloff: FogFalloff::Exponential { density: 0.01 },
                ..default()
            },
        ))
        .insert(InheritedVisibility::VISIBLE)
        //gun model
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("models/p226_anim.glb#Scene0"),
                    transform: Transform::from_xyz(0.1, -0.15, -0.2),
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
                RayCaster::new(Vec3::ZERO, Direction3d::NEG_Z).with_max_hits(1),
                Name::new("RayCast"),
            ));
        });
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //gun animation load
    commands.insert_resource(Animations(vec![
        asset_server.load("models/p226_anim.glb#Animation0"),
        asset_server.load("models/p226_anim.glb#Animation1"),
    ]));

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
