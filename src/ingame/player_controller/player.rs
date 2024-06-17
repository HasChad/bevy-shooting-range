use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use std::f32::consts::PI;

use crate::ingame::GameSettings;
use crate::ingame::WeaponPromp;

#[derive(Component)]
pub struct BulletSpawnPosition;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Head;

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    //player body
    let _player_object_id = commands
        .spawn((
            Player,
            InheritedVisibility::VISIBLE,
            RigidBody::Dynamic,
            Collider::capsule(0.5, 0.25),
            TransformBundle::from(Transform::from_xyz(0.0, 0.5, 0.0)),
            GravityScale(2.0),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            LockedAxes::ROTATION_LOCKED,
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min), //can be changed with air friction
        ))
        .insert(Name::new("Player"))
        .id();

    //let query_filter = SpatialQueryFilter::from_mask(0b1011).with_excluded_entities([player_object_id]);

    //player head
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: settings.fov / 180.0 * PI, // ! One PI = 180, first value is the real fov
                    near: 0.01,
                    ..default()
                }),
                camera: Camera {
                    hdr: true, // HDR is required for bloom
                    ..default()
                },
                ..default()
            },
            FogSettings {
                color: Color::BLACK,
                falloff: FogFalloff::Exponential { density: 0.01 },
                ..default()
            },
            // Enable bloom for the camera
            BloomSettings::NATURAL,
        ))
        .insert(InheritedVisibility::VISIBLE)
        .insert(Name::new("Head"))
        .insert(Head)
        .with_children(|parent| {
            //bullet spawn position
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, -0.5)),
                BulletSpawnPosition,
                Name::new("Bullet Spawn Position"),
            ));
            //gun model
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("models/weapons/P226.glb#Scene0"),
                    transform: Transform::from_xyz(0.1, -0.05, -0.15),
                    ..default()
                },
                WeaponPromp::p226(),
                Name::new("Weapon"),
            ));
            //RayCast

            /*
            parent.spawn((
                RayCaster::new(Vec3::ZERO, Direction3d::NEG_Z)
                    .with_query_filter(query_filter)
                    .with_max_hits(1)
                    .with_max_time_of_impact(2.0)
                    .with_solidness(false),
                Name::new("RayCast"),
            ));
            */
        });
}
