use bevy::prelude::*;
use bevy_xpbd_3d::components::LinearVelocity;

use super::player_controller::player::Player;

#[derive(Component)]
pub struct VelocityText;

#[derive(Component)]
pub struct AmmoText;

pub fn setup_ui(mut commands: Commands) {
    //Ammo UI
    commands.spawn((
        TextBundle::from_section(
            "Ammo/Max",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            margin: UiRect {
                left: Val::Px(400.0),
                ..default()
            },
            ..default()
        }),
        AmmoText,
        Name::new("Ammo Counter"),
    ));
}

pub fn setup_velocity_counter(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
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
        Name::new("Velocity Counter"),
    ));
}

pub fn velocity_update_system(
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
