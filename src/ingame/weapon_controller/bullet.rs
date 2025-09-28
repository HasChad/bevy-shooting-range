#![allow(clippy::too_many_arguments)]

use avian3d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

use super::{HitConfirmEvent, Weapon, WeaponShootingEvent};
use crate::ingame::player::{BulletSpawnPosition, Head, Player};

#[derive(Component)]
pub struct Bullet {
    bullet_lifetime: Timer,
    velocity: Vec3,
}

pub fn spawn_bullet(
    mut commands: Commands,
    mut event_reader: EventReader<WeaponShootingEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawn_position: Single<&GlobalTransform, (With<BulletSpawnPosition>, Without<Head>)>,
    head_transform: Single<&Transform, With<Head>>,
    //
    query: Query<(&Transform, Entity), With<Weapon>>,
    children: Query<&Children>,
    names: Query<&Name>,
    position: Query<&Transform>,
) {
    for _event in event_reader.read() {
        let bullet_velocity =
            (spawn_position.translation() - head_transform.translation).normalize();

        let mut tracer_position: Vec3 = Vec3::ZERO;

        for (gun_trans, entity) in query.iter() {
            for child in children.iter_descendants(entity) {
                if let Ok(name) = names.get(child) {
                    if name.contains("tracer_point") {
                        if let Ok(transform) = position.get(child) {
                            tracer_position = transform.translation + gun_trans.translation;
                            tracer_position.z -= 0.5;
                        }
                    }
                }
            }
        }

        commands
            .spawn((
                Bullet {
                    bullet_lifetime: Timer::from_seconds(3., TimerMode::Once),
                    velocity: bullet_velocity,
                },
                head_transform.clone(),
                InheritedVisibility::VISIBLE,
                Name::new("Bullet"),
            ))
            .with_child((
                Mesh3d(meshes.add(Capsule3d::new(0.002, 1.0))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(1., 0.8, 0.),
                    emissive: LinearRgba {
                        red: 1000.,
                        green: 800.,
                        blue: 0.,
                        alpha: 255.,
                    },
                    ..default()
                })),
                Transform::from_translation(tracer_position)
                    .with_rotation(Quat::from_rotation_x(PI / 2.)),
            ));
    }
}

pub fn bullet_controller(
    time: Res<Time>,
    mut commands: Commands,
    spatial_query: SpatialQuery,
    mut event_writer: EventWriter<HitConfirmEvent>,
    children_query: Query<&Children>,
    player_id: Single<Entity, With<Player>>,
    mut bullet_query: Query<(&mut Transform, &mut Bullet, Entity)>,
    mut transforms: Query<&mut Transform, Without<Bullet>>,
) {
    for (mut bullet_transform, mut bullet_promp, bullet_entity) in bullet_query.iter_mut() {
        let bullet_travel = bullet_promp.velocity * 100.0 * time.delta_secs();
        let distance = (bullet_travel).length();

        let prev_pos = bullet_transform.translation;
        bullet_transform.translation += bullet_travel;

        if let Some(hit) = spatial_query.cast_ray(
            prev_pos,
            Dir3::new_unchecked(bullet_promp.velocity.normalize()),
            distance,
            true,
            &SpatialQueryFilter::from_mask(0b1011).with_excluded_entities([*player_id]),
        ) {
            commands.entity(bullet_entity).despawn();

            event_writer.write(HitConfirmEvent {
                hit_entity: hit.entity,
                hit_normal: hit.normal,
            });
        }

        for child in children_query.iter_descendants(bullet_entity) {
            if let Ok(mut transform) = transforms.get_mut(child) {
                transform
                    .translation
                    .smooth_nudge(&Vec3::ZERO, 2.0, time.delta_secs());
            }
        }

        bullet_promp.bullet_lifetime.tick(time.delta());
        if bullet_promp.bullet_lifetime.finished() {
            commands.entity(bullet_entity).despawn();
        }
    }
}
