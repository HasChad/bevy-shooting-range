use bevy::prelude::*;
use bevy_xpbd_3d::plugins::spatial_query::{SpatialQuery, SpatialQueryFilter};
use std::f32::consts::PI;

use super::{
    player_controller::player::{BulletSpawnPosition, Head},
    WeaponShootingEvent,
};

#[derive(Event)]
pub struct HitConfirmEvent {
    pub hit_entity: Entity,
    pub hit_normal: Vec3,
}

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
    position_query: Query<&GlobalTransform, (With<BulletSpawnPosition>, Without<Head>)>,
    head_query: Query<&Transform, With<Head>>,
) {
    for _event in event_reader.read() {
        let spawn_position = position_query.single().compute_transform();
        let head_transform = head_query.single();
        let bullet_velocity = (spawn_position.translation - head_transform.translation).normalize();
        //bullet_velocity.x += thread_rng().gen_range(-0.002..0.002);

        commands
            .spawn((
                Bullet {
                    bullet_lifetime: Timer::from_seconds(3., TimerMode::Once),
                    velocity: bullet_velocity,
                },
                TransformBundle::from(spawn_position),
                InheritedVisibility::VISIBLE,
                Name::new("Bullet"),
            ))
            .with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Capsule3d::new(0.01, 0.3)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(1.0, 0.8, 0.0),
                        emissive: Color::rgb_linear(23000.0, 0.0, 10000.0),
                        ..default()
                    }),
                    transform: Transform::from_translation(Vec3::new(0.2, -0.13, -0.5))
                        .with_rotation(Quat::from_rotation_x(PI / 2.)),
                    ..default()
                });
            });
    }
}

pub fn bullet_controller(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(&mut Transform, &mut Bullet, Entity)>,
    spatial_query: SpatialQuery,
    mut event_writer: EventWriter<HitConfirmEvent>,
    children_query: Query<&Children>,
    mut transforms: Query<&mut Transform, Without<Bullet>>,
) {
    for (mut bullet_transform, mut bullet_promp, bullet_entity) in bullet_query.iter_mut() {
        let bullet_travel = bullet_promp.velocity * 100.0 * time.delta_seconds();
        let distance = (bullet_travel).length();

        let prev_pos = bullet_transform.translation;
        bullet_transform.translation += bullet_travel;

        //gravity drop
        // bullet_transform.translation.y -= 0.5 * time.delta_seconds() * bullet_promp.bullet_lifetime.elapsed().as_secs_f32();
        //wind push
        // bullet_transform.translation.x -= 0.5 * time.delta_seconds() * bullet_promp.bullet_lifetime.elapsed().as_secs_f32();

        if let Some(hit) = spatial_query.cast_ray(
            prev_pos,
            Direction3d::new_unchecked(bullet_promp.velocity.normalize()),
            distance,
            true,
            SpatialQueryFilter::default(),
        ) {
            commands.entity(bullet_entity).despawn_recursive();

            event_writer.send(HitConfirmEvent {
                hit_entity: hit.entity,
                hit_normal: hit.normal,
            });
        }

        for child in children_query.iter_descendants(bullet_entity) {
            if let Ok(mut transform) = transforms.get_mut(child) {
                transform.translation *= 0.9;
            }
        }

        //bullet despawner
        bullet_promp.bullet_lifetime.tick(time.delta());
        if bullet_promp.bullet_lifetime.finished() {
            commands.entity(bullet_entity).despawn_recursive();
        }
    }
}
