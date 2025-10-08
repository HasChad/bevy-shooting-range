use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::random_range;
use std::f32::consts::PI;

use crate::ingame::ingame_ui::HitmarkerMessage;

#[derive(Component)]
pub struct HitMarker {
    hitmarker_lifetime: Timer,
}

#[derive(Component)]
pub struct CrossParent;

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
            length: 5.0,
            thickness: 2.0,
            gap: 5.0,
            color: Color::WHITE,
            enable: Visibility::Visible,
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
            Transform::from_translation(Vec3::ZERO),
            BackgroundColor(crosshair_settings.color),
            crosshair_settings.enable,
            ZIndex(2),
            CrossParent,
            Name::new("UI - Crosshair"),
        ))
        //MARK: Horizontal Lines
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..default()
                    },
                    Transform::from_translation(Vec3::ZERO),
                    Name::new("Horizontal Lines"),
                ))
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
                        CrosshairLine,
                        BackgroundColor(WHITE.into()),
                        Name::new("Left Line"),
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
                        CrosshairLine,
                        BackgroundColor(WHITE.into()),
                        Name::new("Right Line"),
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
                    Name::new("Vertical Lines"),
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
                        CrosshairLine,
                        BackgroundColor(WHITE.into()),
                        Name::new("Top Line"),
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
                        CrosshairLine,
                        BackgroundColor(WHITE.into()),
                        Name::new("Bottom Line"),
                    ));
                });
        });
}

//MARK: Hitmarker
pub fn hitmarker_spawner(mut commands: Commands, mut mes_reader: MessageReader<HitmarkerMessage>) {
    for _ in mes_reader.read() {
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

        if hitmarker_promp.hitmarker_lifetime.is_finished() {
            commands.entity(hitmarker_entity).despawn();
        }
    }
}
