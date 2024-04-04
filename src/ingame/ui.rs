use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContexts},
    egui::Align2,
};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    //crosshair test
    commands.spawn((
        ImageBundle {
            image: UiImage::new(asset_server.load("crosshairs/dot-cross.png")),
            style: Style {
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            ..default()
        },
        Name::new("PNGCross"),
    ));
}

pub fn png_crosshair_changer(
    mut png_crosshair_query: Query<&mut UiImage>,
    asset_server: Res<AssetServer>,
    mut contexts: EguiContexts,
) {
    let mut png_crosshair_prop = png_crosshair_query.single_mut();

    egui::Window::new("Test")
        .resizable(false)
        .anchor(Align2::RIGHT_CENTER, (-5.0, -5.0))
        .show(contexts.ctx_mut(), |ui| {
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([10.0, 5.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Plus").clicked() {
                            *png_crosshair_prop =
                                UiImage::new(asset_server.load("crosshairs/plus-cross.png"));
                        }
                        if ui.button("Dot").clicked() {
                            *png_crosshair_prop =
                                UiImage::new(asset_server.load("crosshairs/dot-cross.png"));
                        }
                        if ui.button("Offset").clicked() {
                            *png_crosshair_prop =
                                UiImage::new(asset_server.load("crosshairs/offsetplus-cross.png"));
                        }
                        if ui.button("Triangle").clicked() {
                            *png_crosshair_prop =
                                UiImage::new(asset_server.load("crosshairs/triangle-cross.png"));
                        }
                    })
                })
        });
}
