use std::time::Duration;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_xpbd_3d::components::LinearVelocity;

use crate::ingame::{player::Player, weapons::WeaponPromp, CircleTarget};

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
    //MARK: Ammo UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: 50.0,
                ..default()
            }),
            TextSection::new(
                "/",
                TextStyle {
                    font_size: 50.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 30.0,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            margin: UiRect {
                left: Val::Px(300.0),
                ..default()
            },
            ..default()
        })
        .with_background_color(Color::rgba(0.0, 0.0, 0.0, 0.5)),
        AmmoText,
        Name::new("UI - Ammo Counter"),
    ));

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
                color: Color::GOLD,
                ..default()
            }),
            TextSection::new(
                " / 30",
                TextStyle {
                    font_size: 25.0,
                    color: Color::GOLD,
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
                color: Color::GOLD,
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
        .with_background_color(Color::rgba(0.0, 0.0, 0.0, 0.5)),
        TargetCounterText,
        Name::new("UI - Target Counter"),
    ));

    //MARK: Weapon Name UI
    commands.spawn((
        TextBundle::from_sections([TextSection::from_style(TextStyle {
            font_size: 30.0,
            color: Color::GOLD,
            ..default()
        })])
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            margin: UiRect {
                left: Val::Px(300.0),
                bottom: Val::Px(55.0),
                ..default()
            },
            ..default()
        })
        .with_background_color(Color::rgba(0.0, 0.0, 0.0, 0.5)),
        WeaponNameText,
        Name::new("UI - Weapon Name"),
    ));

    //MARK: Veclocity UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Velocity: ",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::GOLD,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::End,
            ..default()
        })
        .with_background_color(Color::BLACK),
        VelocityText,
        Name::new("UI - Velocity Counter"),
    ));

    //MARK: FPS UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::GOLD,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::End,
            ..default()
        })
        .with_background_color(Color::BLACK),
        FpsText,
        Name::new("UI - FPSCounter"),
    ));
}

//MARK: Updaters
pub fn ammo_text_updater(
    mut query: Query<&mut Text, With<AmmoText>>,
    weapon_query: Query<&mut WeaponPromp>,
) {
    for mut text in &mut query {
        for weapon_promp in weapon_query.iter() {
            text.sections[0].value = format!("{}", weapon_promp.mag_capacity);
            text.sections[2].value = format!("{}", weapon_promp.ammo_capacity);
        }
    }
}

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

pub fn weapon_name_text_updater(
    mut query: Query<&mut Text, With<WeaponNameText>>,
    weapon_query: Query<&mut WeaponPromp>,
) {
    for mut text in &mut query {
        for weapon_promp in weapon_query.iter() {
            text.sections[0].value = weapon_promp.name.to_string();
        }
    }
}

pub fn velocity_text_updater(
    query_player: Query<&LinearVelocity, With<Player>>,
    mut query: Query<&mut Text, With<VelocityText>>,
) {
    for mut text in &mut query {
        for linear_velocity in query_player.iter() {
            let sum_velocity = ((linear_velocity.x * linear_velocity.x)
                + (linear_velocity.z * linear_velocity.z))
                .sqrt();
            text.sections[1].value = format!("{sum_velocity:.1}");
        }
    }
}

pub fn fps_text_updater(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.0}");
            }
        }
    }
}
