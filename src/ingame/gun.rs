use bevy::{animation::RepeatAnimation, prelude::*, window::CursorGrabMode};

use crate::ingame::Animations;

pub fn run_animation(
    animations: Res<Animations>,
    mut gun_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut gun in &mut gun_query {
        gun.play(animations.0[0].clone_weak()).repeat();
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
