use bevy::{animation::RepeatAnimation, prelude::*, window::CursorGrabMode};
use bevy_xpbd_3d::prelude::*;

use super::P226;
use crate::ingame::Animations;

#[derive(Event)]
pub struct P226ShootingEvent;

pub fn shooting_event(
    input: Res<ButtonInput<MouseButton>>,
    mut event_writer: EventWriter<P226ShootingEvent>,
    mut p226_query: Query<&mut P226>,
    windows: Query<&Window>,
) {
    for window in windows.iter() {
        if window.cursor.grab_mode == CursorGrabMode::Confined {
            for mut p226 in p226_query.iter_mut() {
                if input.just_pressed(MouseButton::Left) && p226.okay_to_shoot {
                    event_writer.send(P226ShootingEvent);
                    p226.okay_to_shoot = false;
                }
            }
        }
    }
}

pub fn p226_firerate_timer(mut p226: Query<&mut P226>, time: Res<Time>) {
    for mut p226 in p226.iter_mut() {
        if !p226.okay_to_shoot {
            p226.lifetime.tick(time.delta());

            if p226.lifetime.finished() {
                p226.okay_to_shoot = true;
                p226.lifetime = Timer::from_seconds(0.16, TimerMode::Once);
            }
        }
    }
}

pub fn p226_animation_setup(
    animations: Res<Animations>,
    mut gun_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut gun in &mut gun_query {
        gun.play(animations.0[1].clone_weak()).repeat();
        gun.set_repeat(RepeatAnimation::Count(0));
    }
}

pub fn p226_play_animation(
    mut event_reader: EventReader<P226ShootingEvent>,
    mut gun_query: Query<&mut AnimationPlayer>,
) {
    for _event in event_reader.read() {
        for mut gun in &mut gun_query {
            gun.set_repeat(RepeatAnimation::Count(1));
            gun.replay();
        }
    }
}

pub fn print_hits(
    raycast_query: Query<(&RayCaster, &RayHits)>,
    mut event_reader: EventReader<P226ShootingEvent>,
    query: Query<&Name>,
) {
    for _event in event_reader.read() {
        for (ray, hits) in &raycast_query {
            for hit in hits.iter() {
                if let Ok(name) = query.get(hit.entity) {
                    println!("Collider = {}", name);
                }
                /*
                println!(
                    "Hit entity {:?} at {} with normal {}",
                    hit.entity,
                    ray.origin + *ray.direction * hit.time_of_impact,
                    hit.normal,
                );
                */
            }
        }
    }
}
