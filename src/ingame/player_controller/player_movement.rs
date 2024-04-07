use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_xpbd_3d::prelude::*;

use crate::ingame::player_controller::player::Player;
use crate::ingame::player_controller::KeyBindings;
use crate::ingame::GameSettings;

#[derive(Component)]
pub struct VelocityText;

#[derive(Resource)]
pub struct MovementControl {
    pub fmove: f32,
    pub smove: f32,
}

pub fn player_move(
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut movement_controlr: ResMut<MovementControl>,
    input: Res<ButtonInput<KeyCode>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    key_bindings: Res<KeyBindings>,
    mut query_player: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut linear_velocity, player_transform) in query_player.iter_mut() {
            let (yaw_player, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);

            let forward_speed = vector_normalize(Vec3::new(
                -settings.player_speed * yaw_player.sin() * time.delta_seconds(),
                0.0,
                -settings.player_speed * yaw_player.cos() * time.delta_seconds(),
            ));

            let right_speed = vector_normalize(Vec3::new(
                -settings.player_speed * yaw_player.cos() * time.delta_seconds(),
                0.0,
                settings.player_speed * yaw_player.sin() * time.delta_seconds(),
            ));

            match window.cursor.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    //movement
                    for key in input.get_pressed() {
                        let key = *key;

                        if key == key_bindings.move_forward {
                            movement_controlr.fmove = 1.0
                        } else if key == key_bindings.move_backward {
                            movement_controlr.fmove = -1.0
                        }

                        if key == key_bindings.move_left {
                            movement_controlr.smove = 1.0
                        } else if key == key_bindings.move_right {
                            movement_controlr.smove = -1.0
                        }

                        if input.just_pressed(key_bindings.jump) {
                            linear_velocity.y = 5.0;
                        }
                    }

                    for key in input.get_just_released() {
                        let key = *key;

                        if key == key_bindings.move_forward || key == key_bindings.move_backward {
                            movement_controlr.fmove = 0.0
                        }

                        if key == key_bindings.move_left || key == key_bindings.move_right {
                            movement_controlr.smove = 0.0
                        }
                    }
                }
            }

            linear_velocity.x = (movement_controlr.fmove * forward_speed.x)
                + (movement_controlr.smove * right_speed.x);
            linear_velocity.z = (movement_controlr.fmove * forward_speed.z)
                + (movement_controlr.smove * right_speed.z);
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
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
            // Update the value of the second section
            let sum_velocity = linear_velocity.x + linear_velocity.z;
            text.sections[1].value = format!("{sum_velocity:.1}");
        }
    }
}

fn vector_normalize(mut v: Vec3) -> Vec3 {
    let mut length: f32;

    length = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    length = length.sqrt(); // FIXME

    let ilength = 1.0 / length;

    v[0] *= ilength;
    v[1] *= ilength;
    v[2] *= ilength;

    v
}
