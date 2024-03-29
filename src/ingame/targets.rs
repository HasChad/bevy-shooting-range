use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use rand::prelude::*;

use super::P226ShootingEvent;

#[derive(Component)]
pub struct CircleTarget;

#[derive(Component)]
pub struct SilhouetteTarget {
    health: i8,
}

#[derive(Component)]
pub struct SilhouetteTargetHostage {
    health: i8,
}

pub fn target_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //circle target
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/circle_target.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.8, -4.0),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        Name::new("CircleTarget"),
        CircleTarget,
    ));

    //silhouette target
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/silhouette-target.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.8, -3.0),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        Name::new("SilhouetteTarget"),
        SilhouetteTarget { health: 5 },
    ));

    //silhouette target hostage
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/silhouette-target-hostage.glb#Scene0"),
            transform: Transform::from_xyz(1.0, 0.8, -3.0),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::TriMesh)),
        Name::new("SilhouetteTargetHostage"),
        SilhouetteTargetHostage { health: 5 },
    ));
}

pub fn silhouette_target_controller(
    mut commands: Commands,
    raycast_query: Query<&RayHits>,
    mut event_reader: EventReader<P226ShootingEvent>,
    mut silhouettetarget_query: Query<(&mut SilhouetteTarget, Entity)>,
    query: Query<&Name>,
) {
    for _event in event_reader.read() {
        for hits in &raycast_query {
            for hit in hits.iter() {
                match query.get(hit.entity).unwrap().as_str() {
                    "silhouette-target-head" => {
                        for (mut silhotarget_prop, _) in &mut silhouettetarget_query {
                            silhotarget_prop.health -= 3;
                            info!("{}", silhotarget_prop.health);
                        }
                    }
                    "silhouette-target-body" => {
                        for (mut silhotarget_prop, _) in &mut silhouettetarget_query {
                            silhotarget_prop.health -= 1;
                            info!("{}", silhotarget_prop.health);
                        }
                    }
                    _ => (),
                };
                for (silhotarget_prop, silhouettetarget_entity) in &mut silhouettetarget_query {
                    if silhotarget_prop.health <= 0 {
                        commands.entity(silhouettetarget_entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn silhouette_target_hostage_controller(
    mut commands: Commands,
    raycast_query: Query<&RayHits>,
    mut event_reader: EventReader<P226ShootingEvent>,
    mut silhouettetarget_query: Query<(&mut SilhouetteTargetHostage, Entity)>,
    query: Query<&Name>,
) {
    for _event in event_reader.read() {
        for hits in &raycast_query {
            for hit in hits.iter() {
                match query.get(hit.entity).unwrap().as_str() {
                    "silhouette-target-gun-head" => {
                        for (mut silhotarget_prop, _) in &mut silhouettetarget_query {
                            silhotarget_prop.health -= 3;
                            info!("{}", silhotarget_prop.health);
                        }
                    }
                    "silhouette-target-gun-body" => {
                        for (mut silhotarget_prop, _) in &mut silhouettetarget_query {
                            silhotarget_prop.health -= 1;
                            info!("{}", silhotarget_prop.health);
                        }
                    }
                    "hostage" => info!("you shot the hostage"),
                    _ => (),
                };
                for (silhotarget_prop, silhouettetarget_entity) in &mut silhouettetarget_query {
                    if silhotarget_prop.health <= 0 {
                        commands.entity(silhouettetarget_entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub fn circle_target_controller(
    raycast_query: Query<&RayHits>,
    mut event_reader: EventReader<P226ShootingEvent>,
    mut circletarget_query: Query<&mut Transform, With<CircleTarget>>,
    query: Query<&Name>,
) {
    for _event in event_reader.read() {
        for hits in &raycast_query {
            for hit in hits.iter() {
                if "Cylinder" == query.get(hit.entity).unwrap().as_str() {
                    for mut circletarget_entity in &mut circletarget_query {
                        circletarget_entity.translation =
                            Vec3::new(rand::thread_rng().gen_range(-3.0..3.0), 0.8, -4.0);
                    }
                }
            }
        }
    }
}
