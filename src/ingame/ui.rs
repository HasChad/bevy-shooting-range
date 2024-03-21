use bevy::prelude::*;

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
