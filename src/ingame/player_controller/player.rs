use avian3d::prelude::*;
use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use std::f32::consts::PI;

use crate::ingame::{weapons::WeaponPromp, GameSettings};

#[derive(Component)]
pub struct BulletSpawnPosition;

#[derive(Component)]
pub struct Player {
    pub on_ground: bool,
}

#[derive(Component)]
pub struct Head {
    pub current_weapon: WeaponPromp,
}

#[derive(Component)]
pub struct GroundChecker;

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    commands
        .spawn((
            Player { on_ground: true },
            RigidBody::Dynamic,
            Collider::capsule(0.25, 0.5),
            Transform::from_xyz(0.0, 1.0, 0.0),
            TransformInterpolation,
            GravityScale(2.0),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
            Name::new("Player"),
        ))
        .with_child((
            GroundChecker,
            Sensor,
            Collider::cylinder(0.25, 0.001),
            Transform::from_xyz(0.0, -0.5, 0.0),
            Name::new("Ground Checker"),
        ));

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
            Head {
                current_weapon: WeaponPromp::p226(),
            },
        ))
        .with_child((
            // bullet spawn position
            Transform::from_xyz(0.0, 0.0, -0.5),
            BulletSpawnPosition,
            Name::new("Bullet Spawn Position"),
        ))
        .with_child((
            // gun model
            SceneRoot(asset_server.load("models/weapons/P226.glb#Scene0")),
            Transform::from_xyz(0.1, -0.05, -0.15),
            WeaponPromp::p226(),
            Name::new("Weapon"),
        ));

    /*
    //RayCast
    .with_child((
        RayCaster::new(Vec3::ZERO, Direction3d::NEG_Z)
            .with_query_filter(query_filter)
            .with_max_hits(1)
            .with_max_time_of_impact(2.0)
            .with_solidness(false),
        Name::new("RayCast"),
    ));
    */
}
