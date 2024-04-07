use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct P226 {
    pub head_dmg: u8,
    pub body_dmg: u8,
    pub magazine: u8,
    pub lifetime: Timer,
    pub okay_to_shoot: bool,
}

use crate::ingame::GameSettings;

#[derive(Component)]
pub struct Player;

pub fn player_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    //Player

    commands
        .spawn((
            Player,
            InheritedVisibility::VISIBLE,
            RigidBody::Dynamic,
            Collider::capsule(0.5, 0.25),
            TransformBundle::from(Transform::from_xyz(0.0, 1.0, 0.0)),
            GravityScale(2.0),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            LockedAxes::ROTATION_LOCKED,
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min), //can be changed with air friction
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Camera3dBundle {
                        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
                            head_dmg: 3,
                            body_dmg: 1,
                            magazine: 12,
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
        });
}
