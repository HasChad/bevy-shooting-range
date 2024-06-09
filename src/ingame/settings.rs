use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContexts},
    egui::Align2,
};

use super::GameSettings;
use crate::ingame::crosshair;
use crosshair::*;

type InnerLineQueryType<'a> = (&'a mut Style, &'a mut Visibility, &'a mut BackgroundColor);

pub fn egui_settings(
    mut settings: ResMut<GameSettings>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    mut contexts: EguiContexts,
    mut crosshair_inner_settings: ResMut<InnerLineSettings>,
    mut innerhorizontal_query: Query<InnerLineQueryType, With<InnerLineHorizontal>>,
    mut innervertical_query: Query<
        InnerLineQueryType,
        (With<InnerLineVertical>, Without<InnerLineHorizontal>),
    >,
) {
    egui::Window::new("SETTINGS")
        .resizable(false)
        .anchor(Align2::RIGHT_BOTTOM, (-5.0, -5.0))
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([10.0, 5.0])
                .striped(true)
                .show(ui, |ui| {
                    //Game Settings
                    ui.heading("- Game Settings -");
                    ui.end_row();

                    //MARK: Sensitivity
                    ui.label("Sensitivity: ");
                    ui.add(
                        egui::Slider::new(&mut settings.sensitivity, 0.1..=2.0)
                            .trailing_fill(true)
                            .step_by(0.1),
                    );
                    ui.end_row();

                    //MARK: Fov
                    let Projection::Perspective(persp) = camera_query.single_mut().into_inner()
                    else {
                        return;
                    };

                    ui.label("Fov: ");
                    if ui
                        .add(
                            egui::Slider::new(&mut settings.fov, 5.0..=175.0)
                                .trailing_fill(true)
                                .step_by(5.0)
                                .integer(),
                        )
                        .changed()
                    {
                        persp.fov = settings.fov / 180.0 * PI;
                    };
                    ui.end_row();

                    //Crosshair Settings
                    ui.heading("- Crosshair Settings -");
                    ui.end_row();

                    //MARK: Enable
                    let mut enable_check = crosshair_inner_settings.enable == Visibility::Inherited;
                    ui.label("Enable");
                    if ui.checkbox(&mut enable_check, "").changed() {
                        for (_, mut visib, _) in innerhorizontal_query.iter_mut() {
                            if *visib == Visibility::Inherited {
                                *visib = Visibility::Hidden;
                                crosshair_inner_settings.enable = Visibility::Hidden;
                            } else {
                                *visib = Visibility::Inherited;
                                crosshair_inner_settings.enable = Visibility::Inherited;
                            }
                        }
                        for (_, mut visib, _) in innervertical_query.iter_mut() {
                            if *visib == Visibility::Inherited {
                                *visib = Visibility::Hidden;
                                crosshair_inner_settings.enable = Visibility::Hidden;
                            } else {
                                *visib = Visibility::Inherited;
                                crosshair_inner_settings.enable = Visibility::Inherited;
                            }
                        }
                    };
                    ui.end_row();

                    //MARK: Color
                    ui.label("Color: ");
                    let cross_color: Vec3 = crosshair_inner_settings.color.rgb_to_vec3();
                    let mut new_one: [f32; 3] = [cross_color[0], cross_color[1], cross_color[2]];
                    if ui.color_edit_button_rgb(&mut new_one).changed() {
                        crosshair_inner_settings.color =
                            Color::rgba(new_one[0], new_one[1], new_one[2], 1.0);

                        for (_, _, mut node_color) in innerhorizontal_query.iter_mut() {
                            *node_color =
                                bevy::prelude::BackgroundColor(crosshair_inner_settings.color);
                        }
                        for (_, _, mut node_color) in innervertical_query.iter_mut() {
                            *node_color =
                                bevy::prelude::BackgroundColor(crosshair_inner_settings.color);
                        }
                    };
                    ui.end_row();

                    //MARK: Length
                    ui.label("Length: ");
                    if ui
                        .add(
                            egui::Slider::new(&mut crosshair_inner_settings.length, 1.0..=50.0)
                                .trailing_fill(true)
                                .step_by(1.0)
                                .integer(),
                        )
                        .changed()
                    {
                        for (mut style, _, _) in innerhorizontal_query.iter_mut() {
                            style.width = Val::Px(crosshair_inner_settings.length)
                        }

                        for (mut style, _, _) in innervertical_query.iter_mut() {
                            style.height = Val::Px(crosshair_inner_settings.length)
                        }
                    };
                    ui.end_row();

                    //MARK: Thickness
                    ui.label("Thickness: ");
                    if ui
                        .add(
                            egui::Slider::new(&mut crosshair_inner_settings.thickness, 1.0..=50.0)
                                .trailing_fill(true)
                                .step_by(1.0)
                                .integer(),
                        )
                        .changed()
                    {
                        for (mut style, _, _) in innerhorizontal_query.iter_mut() {
                            style.height = Val::Px(crosshair_inner_settings.thickness)
                        }

                        for (mut style, _, _) in innervertical_query.iter_mut() {
                            style.width = Val::Px(crosshair_inner_settings.thickness)
                        }
                    };
                    ui.end_row();

                    //MARK: Offset
                    ui.label("Offset: ");
                    if ui
                        .add(
                            egui::Slider::new(&mut crosshair_inner_settings.offset, 0.0..=50.0)
                                .trailing_fill(true)
                                .step_by(1.0)
                                .integer(),
                        )
                        .changed()
                    {
                        for (mut style, _, _) in innerhorizontal_query.iter_mut() {
                            style.margin.left = Val::Px(crosshair_inner_settings.offset);
                            style.margin.right = Val::Px(crosshair_inner_settings.offset);
                        }

                        for (mut style, _, _) in innervertical_query.iter_mut() {
                            style.margin.top = Val::Px(crosshair_inner_settings.offset);
                            style.margin.bottom = Val::Px(crosshair_inner_settings.offset);
                        }
                    };
                    ui.end_row();
                });
        });
}
