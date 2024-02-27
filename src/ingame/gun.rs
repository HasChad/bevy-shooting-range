use bevy::{animation::RepeatAnimation, prelude::*, window::CursorGrabMode};
use bevy_xpbd_3d::prelude::*;

use super::P226;
use crate::ingame::Animations;

#[derive(Event)]
pub struct P226ShootingEvent;

pub fn run_animation(
    animations: Res<Animations>,
    mut gun_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut gun in &mut gun_query {
        gun.play(animations.0[1].clone_weak()).repeat();
        gun.set_repeat(RepeatAnimation::Count(0));
    }
}

pub fn keyboard_animation_control(
    input: Res<ButtonInput<MouseButton>>,
    mut gun_query: Query<&mut AnimationPlayer>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    for mut gun in &mut gun_query {
        if window.cursor.grab_mode == CursorGrabMode::Confined
            && input.just_pressed(MouseButton::Left)
        {
            gun.set_repeat(RepeatAnimation::Count(1));
            gun.replay();
        }
    }
}

pub fn print_hits(query: Query<(&RayCaster, &RayHits)>, input: Res<ButtonInput<MouseButton>>) {
    if input.just_pressed(MouseButton::Left) {
        for (ray, hits) in &query {
            for hit in hits.iter() {
                println!(
                    "Hit entity {:?} at {} with normal {}",
                    hit.entity,
                    ray.origin + *ray.direction * hit.time_of_impact,
                    hit.normal,
                );
            }
        }
    }
}

pub fn p226_firerate_timer(mut m4: Query<&mut P226>, time: Res<Time>) {
    let mut m4_timer = m4.single_mut();

    if !m4_timer.okay_to_shoot {
        m4_timer.lifetime.tick(time.delta());

        if m4_timer.lifetime.finished() {
            m4_timer.okay_to_shoot = true;
            m4_timer.lifetime = Timer::from_seconds(0.2, TimerMode::Once);
        }
    }
}
