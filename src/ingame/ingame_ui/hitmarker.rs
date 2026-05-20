use bevy::prelude::*;
use rand::random_range;

use crate::ingame::ingame_ui::HitmarkerMessage;

#[derive(Component)]
pub struct HitMarker {
    lifetime: Timer,
}

pub fn spawn_hitmarker(mut commands: Commands) {
    commands
        .spawn((
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,

                ..default()
            },
            Visibility::Hidden,
            HitMarker {
                lifetime: Timer::from_seconds(0.1, TimerMode::Once),
            },
            Name::new("UI - Hitmarker"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Px(10.0),
                    width: Val::Px(2.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    margin: UiRect {
                        left: Val::Px(-1.0),
                        top: Val::Px(-20.0),
                        ..default()
                    },

                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Px(10.0),
                    width: Val::Px(2.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    margin: UiRect {
                        left: Val::Px(-1.0),
                        top: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Px(2.0),
                    width: Val::Px(10.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    margin: UiRect {
                        left: Val::Px(-15.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    height: Val::Px(2.0),
                    width: Val::Px(10.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    margin: UiRect {
                        left: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ));
        });
}

pub fn hitmarker_spawner(
    hitmarker: Single<(&mut HitMarker, &mut Visibility, &mut UiTransform)>,
    mut mes_reader: MessageReader<HitmarkerMessage>,
) {
    let (mut hm_promp, mut hm_vis, mut hm_transform) = hitmarker.into_inner();

    for _ in mes_reader.read() {
        *hm_vis = Visibility::Visible;
        hm_transform.rotation = Rot2::degrees(random_range(35.0..55.0));
        hm_promp.lifetime.reset();
    }
}

pub fn hitmarker_controller(hitmarker: Single<(&mut HitMarker, &mut Visibility)>, time: Res<Time>) {
    let (mut hm_promp, mut hm_vis) = hitmarker.into_inner();

    if hm_promp.lifetime.remaining_secs() > 0.0 {
        hm_promp.lifetime.tick(time.delta());

        if hm_promp.lifetime.is_finished() {
            *hm_vis = Visibility::Hidden;
        }
    }
}
