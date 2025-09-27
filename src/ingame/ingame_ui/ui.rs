use avian3d::prelude::LinearVelocity;
use bevy::{
    color::*,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use palettes::css::GOLD;

use crate::ingame::{player::Player, weapons::Weapon};

#[derive(Component)]
pub struct VelocityText;

#[derive(Component)]
pub struct AmmoText;

#[derive(Component)]
pub struct WeaponNameText;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct TargetCounterText;

pub fn ui_setup(mut commands: Commands) {
    // MARK: Ammo UI
    commands
        .spawn((
            Text::default(),
            TextFont {
                font_size: 50.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::End,
                justify_self: JustifySelf::Center,
                margin: UiRect {
                    left: Val::Px(300.0),
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            Name::new("UI - Ammo Counter"),
            AmmoText,
        ))
        .with_child((
            TextSpan::new("/"),
            TextFont {
                font_size: 50.0,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(GOLD.into()),
        ));

    //MARK: Weapon Name UI
    commands.spawn((
        Text::default(),
        TextFont {
            font_size: 25.0,
            ..default()
        },
        TextColor(GOLD.into()),
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            margin: UiRect {
                left: Val::Px(300.0),
                bottom: Val::Px(65.0),
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        WeaponNameText,
        Name::new("UI - Weapon Name"),
    ));

    //MARK: FPS UI
    commands
        .spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::End,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            Name::new("UI - FPSCounter"),
            FpsText,
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(GOLD.into()),
        ));

    //MARK: Veclocity UI
    commands
        .spawn((
            Text::new("Vel: "),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::End,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            Name::new("UI - Velocity"),
            VelocityText,
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(GOLD.into()),
        ));

    /*
    //MARK: Target Counter UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Kills: ",
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 25.0,
                color: GOLD.into(),
                ..default()
            }),
            TextSection::new(
                " / 30",
                TextStyle {
                    font_size: 25.0,
                    color: GOLD.into(),
                    ..default()
                },
            ),
            TextSection::new(
                "\nTime: ",
                TextStyle {
                    font_size: 25.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 25.0,
                color: GOLD.into(),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            margin: UiRect {
                bottom: Val::Px(100.0),
                ..default()
            },
            ..default()
        })
        .with_background_color(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        TargetCounterText,
        Name::new("UI - Target Counter"),
    ));
    */
}

//MARK: Updaters
pub fn ammo_text_updater(
    mut writer: TextUiWriter,
    weapon: Single<&Weapon>,
    entity: Single<Entity, With<AmmoText>>,
) {
    *writer.text(*entity, 0) = format!("{}", weapon.mag_count);
    *writer.text(*entity, 2) = format!("{}", weapon.ammo_count);
}

pub fn weapon_name_text_updater(
    mut writer: TextUiWriter,
    weapon: Single<&Weapon>,
    entity: Single<Entity, With<WeaponNameText>>,
) {
    *writer.text(*entity, 0) = weapon.name.to_string();
}

pub fn fps_text_updater(
    mut writer: TextUiWriter,
    diagnostics: Res<DiagnosticsStore>,
    entity: Single<Entity, With<FpsText>>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            *writer.text(*entity, 1) = format!("{value:.0}");
        }
    }
}

pub fn velocity_text_updater(
    mut writer: TextUiWriter,
    lin_vel: Single<&LinearVelocity, With<Player>>,
    entity: Single<Entity, With<VelocityText>>,
) {
    let sum_vel = Vec3::new(lin_vel.x, 0.0, lin_vel.z);

    *writer.text(*entity, 1) = format!("{:.2}", sum_vel.length());
}

/*
pub fn target_text_updater(
    time: Res<Time>,
    mut query: Query<&mut Text, With<TargetCounterText>>,
    mut circletarget_query: Query<&mut CircleTarget>,
) {
    for mut text in &mut query {
        for mut circletarget_prop in circletarget_query.iter_mut() {
            text.sections[1].value = format!("{}", circletarget_prop.hit_counter);
            text.sections[4].value = format!("{:.2}", circletarget_prop.timer.elapsed_secs());
            if circletarget_prop.hit_counter > 0 && circletarget_prop.hit_counter < 30 {
                circletarget_prop
                    .timer
                    .tick(Duration::from_secs_f32(time.delta_seconds()));
            }
        }
    }
}
*/
