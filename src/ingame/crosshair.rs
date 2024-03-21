use bevy::prelude::*;

#[derive(Component)]
pub struct InnerLineHorizontal;

#[derive(Component)]
pub struct InnerLineVertical;

#[derive(Resource)]
pub struct InnerLineSettings {
    pub offset: f32,
    pub color: Color,
    pub length: f32,
    pub thickness: f32,
    pub enable: InheritedVisibility,
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
            inherited_visibility: crosshair_settings.enable,
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
            inherited_visibility: crosshair_settings.enable,
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
            inherited_visibility: crosshair_settings.enable,
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
            inherited_visibility: crosshair_settings.enable,
            ..default()
        },
        Name::new("Bottom Line"),
        InnerLineVertical,
    ));
}
