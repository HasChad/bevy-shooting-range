use std::f32::consts::PI;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::HitConfirmEvent;

#[derive(Component)]
pub struct HitMarker {
    hitmarker_lifetime: Timer,
}

#[derive(Component)]
pub struct InnerLineHorizontal;

#[derive(Component)]
pub struct InnerLineVertical;

#[derive(Resource)]
pub struct InnerLineSettings {
    pub length: f32,
    pub thickness: f32,
    pub offset: f32,
    pub color: Color,
    pub enable: Visibility,
}

pub fn crosshair_setup(mut commands: Commands, crosshair_settings: Res<InnerLineSettings>) {
    //Left Line
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Px(crosshair_settings.thickness),
                width: Val::Px(crosshair_settings.length),
                right: Val::Percent(50.0),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::End,
                margin: UiRect {
                    right: Val::Px(crosshair_settings.offset),
                    ..default()
                },
                ..default()
            },
            background_color: crosshair_settings.color.into(),
            visibility: crosshair_settings.enable,
            ..default()
        },
        Name::new("Left Line"),
        InnerLineHorizontal,
    ));

    //Right Line
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Px(crosshair_settings.thickness),
                width: Val::Px(crosshair_settings.length),
                left: Val::Percent(50.0),
                align_self: AlignSelf::Center,
                margin: UiRect {
                    left: Val::Px(crosshair_settings.offset),
                    ..default()
                },
                ..default()
            },
            background_color: crosshair_settings.color.into(),
            visibility: crosshair_settings.enable,

            ..default()
        },
        Name::new("Right Line"),
        InnerLineHorizontal,
    ));

    //Top Line
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Px(crosshair_settings.length),
                width: Val::Px(crosshair_settings.thickness),
                bottom: Val::Percent(50.0),
                align_self: AlignSelf::End,
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    bottom: Val::Px(crosshair_settings.offset),
                    ..default()
                },
                ..default()
            },
            background_color: crosshair_settings.color.into(),
            visibility: crosshair_settings.enable,
            ..default()
        },
        Name::new("Top Line"),
        InnerLineVertical,
    ));

    //Bottom Line
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Px(crosshair_settings.length),
                width: Val::Px(crosshair_settings.thickness),
                top: Val::Percent(50.0),
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    top: Val::Px(crosshair_settings.offset),
                    ..default()
                },
                ..default()
            },
            background_color: crosshair_settings.color.into(),
            visibility: crosshair_settings.enable,
            ..default()
        },
        Name::new("Bottom Line"),
        InnerLineVertical,
    ));
}

pub fn hitmarker_spawner(mut commands: Commands, mut event_reader: EventReader<HitConfirmEvent>) {
    for _event in event_reader.read() {
        commands
            .spawn(NodeBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(
                    PI / thread_rng().gen_range(3.5..4.5),
                )),
                style: Style {
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                ..default()
            })
            .insert(HitMarker {
                hitmarker_lifetime: Timer::from_seconds(0.1, TimerMode::Once),
            })
            .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
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
                    background_color: Color::WHITE.into(),
                    ..default()
                });
                parent.spawn(NodeBundle {
                    style: Style {
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
                    background_color: Color::WHITE.into(),
                    ..default()
                });
                parent.spawn(NodeBundle {
                    style: Style {
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
                    background_color: Color::WHITE.into(),
                    ..default()
                });
                parent.spawn(NodeBundle {
                    style: Style {
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
                    background_color: Color::WHITE.into(),
                    ..default()
                });
            });
    }
}

pub fn hitmarker_controller(
    mut commands: Commands,
    mut hitmarker_query: Query<(&mut HitMarker, Entity)>,
    time: Res<Time>,
) {
    for (mut hitmarker_promp, hitmarker_entity) in hitmarker_query.iter_mut() {
        hitmarker_promp.hitmarker_lifetime.tick(time.delta());

        if hitmarker_promp.hitmarker_lifetime.finished() {
            commands.entity(hitmarker_entity).despawn_recursive();
        }
    }
}
