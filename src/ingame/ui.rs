use bevy::prelude::*;
use bevy_xpbd_3d::components::LinearVelocity;

use super::{player_controller::player::Player, WeaponPromp};

#[derive(Component)]
pub struct VelocityText;

#[derive(Component)]
pub struct AmmoText;

#[derive(Component)]
pub struct WeaponNameText;

pub fn setup_ui(mut commands: Commands) {
    //Ammo UI
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
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
                font_size: 40.0,
                color: Color::WHITE,
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

    //Weapon Name UI
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

    //Veclocity UI
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
}

pub fn ammo_text_updater(
    mut query: Query<&mut Text, With<AmmoText>>,
    p226_query: Query<&mut WeaponPromp>,
) {
    for mut text in &mut query {
        for p226 in p226_query.iter() {
            text.sections[0].value = format!("{}", p226.mag_capacity);
            text.sections[2].value = format!("{}", p226.ammo_capacity);
        }
    }
}

pub fn weapon_name_text_updater(
    mut query: Query<&mut Text, With<WeaponNameText>>,
    p226_query: Query<&mut WeaponPromp>,
) {
    for mut text in &mut query {
        for p226 in p226_query.iter() {
            text.sections[0].value = p226.name.to_string();
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
