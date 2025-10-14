use avian3d::prelude::*;
use bevy::{
    pbr::ScreenSpaceAmbientOcclusion, post_process::bloom::Bloom, prelude::*, render::view::Hdr,
};
use std::f32::consts::PI;

use crate::ingame::{weapons::Weapon, GameSettings};

pub const WEAPON_POS: Vec3 = Vec3::new(0.075, -0.04, -0.1);

#[derive(Component)]
pub struct BulletSpawnPosition;

#[derive(Component)]
pub struct Player {
    pub fmove: i8,
    pub smove: i8,
    pub jump: bool,
    pub walk: bool,
    pub on_ground: bool,
}

#[derive(Component)]
pub struct Head {
    pub current_weapon: Weapon,
}

#[derive(Component)]
pub struct GroundChecker;

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    commands.spawn((
        Player {
            fmove: 0,
            smove: 0,
            jump: false,
            walk: false,
            on_ground: true,
        },
        RigidBody::Kinematic,
        Collider::cylinder(0.25, 1.0),
        Transform::from_xyz(0.0, 0.5, 0.0),
        TransformInterpolation,
        Name::new("Player"),
    ));

    commands
        .spawn((
            Camera3d::default(),
            Hdr,
            ScreenSpaceAmbientOcclusion::default(),
            Msaa::Off,
            Projection::Perspective(PerspectiveProjection {
                fov: settings.fov / 180.0 * PI,
                near: 0.01,
                ..default()
            }),
            Bloom::NATURAL,
            Name::new("Head"),
            Head {
                current_weapon: Weapon::p226(),
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
            Transform::from_translation(WEAPON_POS),
            Weapon::p226(),
            Name::new("Weapon"),
        ));
}
