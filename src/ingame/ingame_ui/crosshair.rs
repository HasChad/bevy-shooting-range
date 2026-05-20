use bevy::{color::palettes::css::WHITE, prelude::*};

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
            UiTransform::from_translation(Val2::ZERO),
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
                    UiTransform::from_translation(Val2::ZERO),
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
                    UiTransform::from_rotation(Rot2::degrees(90.0)),
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
