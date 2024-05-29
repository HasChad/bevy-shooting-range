use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContexts},
    egui::Align2,
};

use super::GameSettings;
use crate::ingame::crosshair;
use crosshair::*;

pub fn egui_settings(
    mut settings: ResMut<GameSettings>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    mut contexts: EguiContexts,
    mut crosshair_inner_settings: ResMut<InnerLineSettings>,
    mut innerhorizontal_query: Query<Entity, With<InnerLineHorizontal>>,
    mut innervertical_query: Query<Entity, With<InnerLineVertical>>,
    mut style_query: Query<&mut Style>,
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

                    ui.label("Sensitivity: ");
                    if ui
                        .add(
                            egui::Slider::new(&mut settings.sensitivity, 0.1..=2.0)
                                .trailing_fill(true)
                                .step_by(0.1),
                        )
                        .changed()
                    {};
                    ui.end_row();

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

                    let mut cross_color: [f32; 4] =
                        crosshair_inner_settings.color.as_linear_rgba_f32();
                    ui.label("Color: ");
                    if ui
                        .color_edit_button_rgba_unmultiplied(&mut cross_color)
                        .changed()
                    {
                        let new_color = Color::rgba(
                            cross_color[0],
                            cross_color[1],
                            cross_color[2],
                            cross_color[3],
                        );
                        crosshair_inner_settings.color = new_color;
                    };
                    ui.end_row();

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
                        for node_style in innerhorizontal_query.iter() {
                            if let Ok(mut style) = style_query.get_mut(node_style) {
                                style.width = Val::Px(crosshair_inner_settings.length)
                            };
                        }

                        for node_style in innervertical_query.iter_mut() {
                            if let Ok(mut style) = style_query.get_mut(node_style) {
                                style.height = Val::Px(crosshair_inner_settings.length)
                            };
                        }
                    };
                    ui.end_row();

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
                        for node_style in innerhorizontal_query.iter_mut() {
                            if let Ok(mut style) = style_query.get_mut(node_style) {
                                style.height = Val::Px(crosshair_inner_settings.thickness)
                            };
                        }

                        for node_style in innervertical_query.iter_mut() {
                            if let Ok(mut style) = style_query.get_mut(node_style) {
                                style.width = Val::Px(crosshair_inner_settings.thickness)
                            };
                        }
                    };
                    ui.end_row();

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
                        for node_style in innerhorizontal_query.iter_mut() {
                            if let Ok(mut style) = style_query.get_mut(node_style) {
                                style.margin.left = Val::Px(crosshair_inner_settings.offset);
                                style.margin.right = Val::Px(crosshair_inner_settings.offset);
                            };
                        }

                        for node_style in innervertical_query.iter_mut() {
                            if let Ok(mut style) = style_query.get_mut(node_style) {
                                style.margin.top = Val::Px(crosshair_inner_settings.offset);
                                style.margin.bottom = Val::Px(crosshair_inner_settings.offset);
                            };
                        }
                    };
                    ui.end_row();
                });
        });
}
