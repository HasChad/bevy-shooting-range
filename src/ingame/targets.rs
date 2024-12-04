use avian3d::prelude::*;
use bevy::{prelude::*, time::Stopwatch};
use bevy_kira_audio::prelude::*;
use rand::prelude::*;

use crate::{
    ingame::{weapons::WeaponPromp, HitConfirmEvent},
    ingame_ui::HitmarkerEvent,
};

#[derive(Component)]
pub struct CircleTarget {
    pub hit_counter: u32,
    pub timer: Stopwatch,
}

#[derive(Component)]
pub struct EnemyTarget {
    health: i8,
}

#[derive(Component)]
pub struct EnemyTargetHostage {
    health: i8,
}

#[derive(Event)]
pub struct CircleTargetEvent {
    entity: Entity,
}

pub fn hit_detector(
    audio: Res<Audio>,
    query: Query<&Name>,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<HitConfirmEvent>,
    mut circletarget_event_writer: EventWriter<CircleTargetEvent>,
    mut hitmarker_event_writer: EventWriter<HitmarkerEvent>,
    parent_query: Query<&Parent>,
) {
    for event in event_reader.read() {
        let hit_entity_name = query.get(event.hit_entity).unwrap().as_str();

        if hit_entity_name == "Cylinder" {
            let ancestor = parent_query
                .iter_ancestors(event.hit_entity)
                .last()
                .unwrap();

            circletarget_event_writer.send(CircleTargetEvent { entity: ancestor });

            hitmarker_event_writer.send(HitmarkerEvent);
            audio.play(asset_server.load("sounds/hitmarker.ogg"));
        }
    }
}

pub fn target_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //circle target
    commands.spawn((
        SceneRoot(asset_server.load("models/targets/circle_target.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, 0.75, -4.0),
        Name::new("CircleTarget"),
        CircleTarget {
            hit_counter: 0,
            timer: Stopwatch::new(),
        },
    ));

    /*
    //enemy target
    commands.spawn((
        SceneRoot(asset_server.load("models/targets/enemy-target.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(0.0, 0.5, -3.0),
        Name::new("EnemyTarget"),
        EnemyTarget { health: 5 },
    ));

    //enemy target hostage
    commands.spawn((
        SceneRoot(asset_server.load("models/targets/enemy-target-hostage.glb#Scene0")),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
        Transform::from_xyz(1.0, 0.8, -3.0),
        Name::new("EnemyTargetHostage"),
        EnemyTargetHostage { health: 5 },
    ));
    */
}

pub fn circle_target_controller(
    mut event_reader: EventReader<CircleTargetEvent>,
    mut circletarget_query: Query<(&mut Transform, &mut CircleTarget, Entity)>,
) {
    for event in event_reader.read() {
        for (mut circletarget_transform, mut circletarget_prop, circletarget_entity) in
            circletarget_query.iter_mut()
        {
            info!("{:?}", event.entity);
            info!("{:?}", circletarget_entity);

            if event.entity == circletarget_entity {
                let old_position = circletarget_transform.translation.x;
                if circletarget_prop.hit_counter < 30 {
                    circletarget_prop.hit_counter += 1;
                }
                while (circletarget_transform.translation.x - old_position).abs() < 0.5 {
                    circletarget_transform.translation.x = thread_rng().gen_range(-3.0..3.0);
                }
            }
        }
    }
}

pub fn enemy_target_controller(
    mut commands: Commands,
    mut event_reader: EventReader<HitConfirmEvent>,
    mut enemytarget_query: Query<(&mut EnemyTarget, Entity)>,
    weapon_query: Query<&WeaponPromp>,
    query: Query<&Name>,
) {
    for event in event_reader.read() {
        let weapon_promp = weapon_query.single();
        match query.get(event.hit_entity).unwrap().as_str() {
            "silhouette-target-head" => {
                for (mut enemytarget_prop, _) in &mut enemytarget_query {
                    enemytarget_prop.health -= weapon_promp.head_damage as i8;
                    info!("{}", enemytarget_prop.health);
                }
            }
            "silhouette-target-body" => {
                for (mut enemytarget_prop, _) in &mut enemytarget_query {
                    enemytarget_prop.health -= weapon_promp.body_damage as i8;

                    info!("{}", enemytarget_prop.health);
                }
            }
            _ => (),
        };
        for (enemytarget_prop, enemytarget_entity) in &mut enemytarget_query {
            if enemytarget_prop.health <= 0 {
                commands.entity(enemytarget_entity).despawn_recursive();
            }
        }
    }
}

pub fn enemy_target_hostage_controller(
    mut commands: Commands,
    mut event_reader: EventReader<HitConfirmEvent>,
    mut enemytargethostage_query: Query<(&mut EnemyTargetHostage, Entity)>,
    weapon_query: Query<&WeaponPromp>,
    query: Query<&Name>,
) {
    for event in event_reader.read() {
        let weapon_promp = weapon_query.single();

        match query.get(event.hit_entity).unwrap().as_str() {
            "silhouette-target-gun-head" => {
                for (mut enemytargethostage_prop, _) in &mut enemytargethostage_query {
                    enemytargethostage_prop.health -= weapon_promp.head_damage as i8;
                    info!("{}", enemytargethostage_prop.health);
                }
            }
            "silhouette-target-gun-body" => {
                for (mut enemytargethostage_prop, _) in &mut enemytargethostage_query {
                    enemytargethostage_prop.health -= weapon_promp.body_damage as i8;
                    info!("{}", enemytargethostage_prop.health);
                }
            }
            "hostage" => info!("you shot the hostage"),
            _ => (),
        };
        for (enemytargethostage_prop, enemytargethostage_entity) in &mut enemytargethostage_query {
            if enemytargethostage_prop.health <= 0 {
                commands
                    .entity(enemytargethostage_entity)
                    .despawn_recursive();
            }
        }
    }
}
