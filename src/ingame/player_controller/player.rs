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
    let player_entity = commands
        .spawn((
            Player { on_ground: true },
            RigidBody::Kinematic,
            Collider::cylinder(0.25, 1.0),
            Transform::from_xyz(0.0, 0.5, 0.0),
            TransformInterpolation,
            Name::new("Player"),
        ))
        .id();

    commands.entity(player_entity).with_child((
        GroundChecker,
        RayCaster::new(Vec3::ZERO, Dir3::NEG_Y)
            .with_query_filter(SpatialQueryFilter::from_excluded_entities([player_entity]))
            .with_max_hits(1)
            .with_max_distance(0.55)
            .with_solidness(false),
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
        // bullet spawn position
        .with_child((
            Transform::from_xyz(0.0, 0.0, -0.5),
            BulletSpawnPosition,
            Name::new("Bullet Spawn Position"),
        ))
        // gun model
        .with_child((
            SceneRoot(asset_server.load("models/weapons/P226.glb#Scene0")),
            Transform::from_xyz(0.1, -0.05, -0.15),
            WeaponPromp::p226(),
            Name::new("Weapon"),
        ));
}
