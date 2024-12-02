use avian3d::prelude::*;
use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use std::f32::consts::PI;

use crate::ingame::{weapons::WeaponPromp, GameSettings};

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
    commands
        .spawn((
            Player,
            RigidBody::Dynamic,
            Collider::capsule(0.25, 0.5),
            Transform::from_xyz(0.0, 1.0, 0.0),
            GravityScale(2.0),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            LockedAxes::ROTATION_LOCKED,
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min), //can be changed with air friction
        ))
        .insert(Name::new("Player"));

    //player head
    commands
        .spawn((
            Camera3d::default(),
            Camera {
                hdr: true, // HDR is required for the bloom effect
                ..default()
            },
            Projection::Perspective(PerspectiveProjection {
                fov: settings.fov / 180.0 * PI,
                near: 0.01,
                ..default()
            }),
            Bloom::NATURAL,
            Name::new("Head"),
            Head,
        ))
        .with_children(|parent| {
            //bullet spawn position
            parent.spawn((
                Transform::from_xyz(0.0, 0.0, -0.5),
                BulletSpawnPosition,
                Name::new("Bullet Spawn Position"),
            ));

            //gun model
            parent.spawn((
                SceneRoot(asset_server.load("models/weapons/P226.glb#Scene0")),
                Transform::from_xyz(0.1, -0.05, -0.15),
                WeaponPromp::p226(),
                Name::new("Weapon"),
            ));

            /*
            //RayCast
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
