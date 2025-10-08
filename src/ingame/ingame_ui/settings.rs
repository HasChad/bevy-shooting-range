use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContext, PrimaryEguiContext},
    egui::{Align2, Color32, RichText, Slider},
};
use std::f32::consts::PI;

use super::{CrosshairLine, CrosshairLineSettings};
use crate::ingame::{ingame_ui::crosshair::CrossParent, GameSettings, PlayableState};

#[derive(Component)]
pub struct SettingsUI;

pub fn setting_bg(mut commands: Commands, mut cross: Single<&mut Node, With<CrossParent>>) {
    cross.margin.right = Val::Px(500.0);

    commands.spawn((
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BackgroundColor(Color::Hsla(Hsla {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.0,
            alpha: 0.7,
        })),
        ZIndex(1),
        SettingsUI,
    ));

    commands.spawn((
        Text::new(concat!(
            "ESC - Enter/Exit play mode\n",
            "W/A/S/D - Move\n",
            "Space - Jump\n",
            "Left Mouse Button - Shoot\n",
            "Right Mouse Button - Aim\n",
            "R - Reload\n",
            "X - Reset player position\n",
            "1 - P226\n",
            "2 - AK-15"
        )),
        ZIndex(2),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::Start,
            margin: UiRect {
                left: Val::Px(5.0),
                top: Val::Px(5.0),
                ..default()
            },
            ..default()
        },
        SettingsUI,
        Name::new("UI - Information Text "),
    ));
}

pub fn despawn_bg(
    mut commands: Commands,
    query: Query<Entity, With<SettingsUI>>,
    mut cross: Single<&mut Node, With<CrossParent>>,
) {
    cross.margin.right = Val::Px(0.0);

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn egui_settings(
    mut next_state: ResMut<NextState<PlayableState>>,
    mut exit: MessageWriter<AppExit>,
    mut cursor_options: Single<&mut CursorOptions>,
    mut egui_context: Single<&mut EguiContext, With<PrimaryEguiContext>>,
    mut settings: ResMut<GameSettings>,
    mut camera_query: Single<&mut Projection, With<Camera3d>>,
    mut crosshair_line_settings: ResMut<CrosshairLineSettings>,
    mut crosshair_line: Query<
        (&mut Node, &mut Visibility, &mut BackgroundColor),
        With<CrosshairLine>,
    >,
) {
    egui::Window::new("SETTINGS")
        .resizable(false)
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
        .show(egui_context.get_mut(), |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([10.0, 5.0])
                .striped(true)
                .show(ui, |ui| {
                    //Game Settings
                    ui.heading(RichText::new("- Game Settings -").color(Color32::WHITE));
                    ui.end_row();

                    ui.label("Sensitivity: ");
                    ui.add(
                        Slider::new(&mut settings.sensitivity, 0.1..=2.0)
                            .trailing_fill(true)
                            .step_by(0.1),
                    );
                    ui.end_row();

                    ui.label("Fov: ");
                    let Projection::Perspective(persp) = camera_query.as_mut() else {
                        return;
                    };
                    if ui
                        .add(
                            Slider::new(&mut settings.fov, 70.0..=130.0)
                                .trailing_fill(true)
                                .step_by(5.0)
                                .integer(),
                        )
                        .changed()
                    {
                        persp.fov = settings.fov / 180.0 * PI;
                    };
                    ui.end_row();
                    ui.end_row();

                    //Crosshair Settings
                    ui.heading(RichText::new("- Crosshair Settings -").color(Color32::WHITE));
                    ui.end_row();

                    let mut enable_check = crosshair_line_settings.enable == Visibility::Visible;
                    ui.label("Enable");
                    if ui.checkbox(&mut enable_check, "").changed() {
                        for (_, mut visib, _) in crosshair_line.iter_mut() {
                            match *visib {
                                Visibility::Hidden => {
                                    *visib = Visibility::Visible;
                                    crosshair_line_settings.enable = Visibility::Visible;
                                }
                                Visibility::Visible => {
                                    *visib = Visibility::Hidden;
                                    crosshair_line_settings.enable = Visibility::Hidden;
                                }
                                _ => (),
                            }
                        }
                    };
                    ui.end_row();

                    ui.label("Color: ");
                    let test = crosshair_line_settings.color.to_linear();
                    let mut new_one: [f32; 3] = [test.red, test.green, test.blue];

                    if ui.color_edit_button_rgb(&mut new_one).changed() {
                        crosshair_line_settings.color = LinearRgba {
                            red: new_one[0],
                            green: new_one[1],
                            blue: new_one[2],
                            alpha: 1.0,
                        }
                        .into();
                        for (_, _, mut bgcolor) in crosshair_line.iter_mut() {
                            *bgcolor = BackgroundColor(crosshair_line_settings.color);
                        }
                    };
                    ui.end_row();

                    ui.label("Length: ");
                    if ui
                        .add(
                            Slider::new(&mut crosshair_line_settings.length, 1.0..=50.0)
                                .trailing_fill(true)
                                .step_by(1.0)
                                .integer(),
                        )
                        .changed()
                    {
                        for (mut style, _, _) in crosshair_line.iter_mut() {
                            style.width = Val::Px(crosshair_line_settings.length)
                        }
                    };
                    ui.end_row();

                    ui.label("Thickness: ");
                    if ui
                        .add(
                            Slider::new(&mut crosshair_line_settings.thickness, 1.0..=50.0)
                                .trailing_fill(true)
                                .step_by(1.0)
                                .integer(),
                        )
                        .changed()
                    {
                        for (mut style, _, _) in crosshair_line.iter_mut() {
                            style.height = Val::Px(crosshair_line_settings.thickness)
                        }
                    };
                    ui.end_row();

                    ui.label("Gap: ");
                    if ui
                        .add(
                            Slider::new(&mut crosshair_line_settings.gap, 0.0..=50.0)
                                .trailing_fill(true)
                                .step_by(1.0)
                                .integer(),
                        )
                        .changed()
                    {
                        for (mut style, _, _) in crosshair_line.iter_mut() {
                            style.margin.left = Val::Px(crosshair_line_settings.gap);
                            style.margin.right = Val::Px(crosshair_line_settings.gap);
                        }
                    };
                    ui.end_row();
                    ui.end_row();

                    /*
                    //Audio Settings
                    ui.heading(RichText::new("- Audio Settings -").color(Color32::WHITE));
                    ui.end_row();

                    ui.label("Volume: ");
                    if ui
                        .add(
                            Slider::new(&mut settings.volume, 0.0..=1.0)
                                .trailing_fill(true)
                                .step_by(0.05),
                        )
                        .changed()
                    {
                        audio.set_volume(settings.volume as f64);
                    }
                    ui.end_row();
                    ui.end_row();
                    */

                    //Quit
                    ui.heading(RichText::new("- Quit Game -").color(Color32::WHITE));
                    ui.end_row();

                    ui.horizontal_centered(|ui| {
                        if ui.button("Quit").clicked() {
                            exit.write(AppExit::Success);
                        }

                        if ui.button("Resume").clicked() {
                            cursor_options.grab_mode = CursorGrabMode::Confined;
                            cursor_options.visible = false;
                            next_state.set(PlayableState::Action)
                        }
                        ui.end_row();
                    });
                });
        });
}
