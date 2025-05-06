use bevy::prelude::*;
use rand::random_range;
use std::f32::consts::PI;

use super::HitmarkerEvent;

#[derive(Component)]
pub struct HitMarker {
    hitmarker_lifetime: Timer,
}

#[derive(Component)]
pub struct CrosshairLine;

#[derive(Resource)]
pub struct CrosshairLineSettings {
    pub length: f32,
    pub thickness: f32,
    pub gap: f32,
    pub color: Color,
    pub enable: Visibility,
}

impl Default for CrosshairLineSettings {
    fn default() -> Self {
        CrosshairLineSettings {
            gap: 5.0,
            color: Color::WHITE,
            length: 5.0,
            thickness: 2.0,
            enable: Visibility::Inherited,
        }
    }
}

pub fn crosshair_setup(mut commands: Commands, crosshair_settings: Res<CrosshairLineSettings>) {
    commands
        .spawn((
            Node {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            Name::new("UI - Crosshair"),
        ))
        //MARK: Horizontal Lines
        .with_children(|parent| {
            parent
                .spawn(Node {
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                })
                .with_children(|parent| {
                    //Left Line
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            height: Val::Px(crosshair_settings.thickness),
                            width: Val::Px(crosshair_settings.length),
                            right: Val::Percent(50.0),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::End,
                            margin: UiRect {
                                right: Val::Px(crosshair_settings.gap),
                                ..default()
                            },
                            ..default()
                        },
                        BackgroundColor(crosshair_settings.color),
                        crosshair_settings.enable,
                        Name::new("Left Line"),
                        CrosshairLine,
                    ));

                    //Right Line
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            height: Val::Px(crosshair_settings.thickness),
                            width: Val::Px(crosshair_settings.length),
                            left: Val::Percent(50.0),
                            align_self: AlignSelf::Center,
                            margin: UiRect {
                                left: Val::Px(crosshair_settings.gap),
                                ..default()
                            },
                            ..default()
                        },
                        BackgroundColor(crosshair_settings.color),
                        crosshair_settings.enable,
                        Name::new("Right Line"),
                        CrosshairLine,
                    ));
                });

            //MARK: Vertical Lines
            parent
                .spawn((
                    Node {
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    Transform::from_rotation(Quat::from_rotation_z(PI / 2.)),
                ))
                .with_children(|parent| {
                    //Top Line
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            height: Val::Px(crosshair_settings.thickness),
                            width: Val::Px(crosshair_settings.length),
                            right: Val::Percent(50.0),
                            align_self: AlignSelf::Center,
                            justify_self: JustifySelf::End,
                            margin: UiRect {
                                right: Val::Px(crosshair_settings.gap),
                                ..default()
                            },
                            ..default()
                        },
                        BackgroundColor(crosshair_settings.color),
                        crosshair_settings.enable,
                        Name::new("Left Line"),
                        CrosshairLine,
                    ));

                    //Bottom Line
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            height: Val::Px(crosshair_settings.thickness),
                            width: Val::Px(crosshair_settings.length),
                            left: Val::Percent(50.0),
                            align_self: AlignSelf::Center,
                            margin: UiRect {
                                left: Val::Px(crosshair_settings.gap),
                                ..default()
                            },
                            ..default()
                        },
                        BackgroundColor(crosshair_settings.color),
                        crosshair_settings.enable,
                        Name::new("Right Line"),
                        CrosshairLine,
                    ));
                });
        });
}

//MARK: Hitmarker
pub fn hitmarker_spawner(mut commands: Commands, mut event_reader: EventReader<HitmarkerEvent>) {
    for _event in event_reader.read() {
        commands
            .spawn((
                Node {
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,

                    ..default()
                },
                Transform::from_rotation(Quat::from_rotation_z(PI / random_range(3.5..4.5))),
                HitMarker {
                    hitmarker_lifetime: Timer::from_seconds(0.1, TimerMode::Once),
                },
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
}

pub fn hitmarker_controller(
    mut commands: Commands,
    mut hitmarker_query: Query<(&mut HitMarker, Entity)>,
    time: Res<Time>,
) {
    for (mut hitmarker_promp, hitmarker_entity) in hitmarker_query.iter_mut() {
        hitmarker_promp.hitmarker_lifetime.tick(time.delta());

        if hitmarker_promp.hitmarker_lifetime.finished() {
            commands.entity(hitmarker_entity).despawn();
        }
    }
}
