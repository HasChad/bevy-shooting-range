use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContexts},
    egui::Align2,
};
use serde::{Deserialize, Serialize};

pub mod gun;
pub mod ingame_setup;
pub mod player;
pub mod targets;

use gun::*;
use ingame_setup::*;
use player::*;
use targets::*;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct GameSettings {
    pub sensitivity: f32,
    pub fov: f32,
}

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup, setup, first_target_setup))
            .add_systems(
                Update,
                (
                    //gun systems
                    shooting_event,
                    p226_firerate_timer,
                    p226_animation_setup,
                    p226_play_animation,
                    print_hits,
                    //player systems
                    player_look,
                    edit_mode_toggler,
                    //target systems
                    circle_target_controller,
                    //mod system
                    change_settings,
                ),
            )
            //plugins
            //resources
            .insert_resource(GameSettings {
                sensitivity: 0.02,
                fov: 90.0,
            })
            .init_resource::<InputState>()
            //events
            .add_event::<P226ShootingEvent>();
    }
}

fn change_settings(
    mut settings: ResMut<GameSettings>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
    mut contexts: EguiContexts,
) {
    let Projection::Perspective(persp) = camera_query.single_mut().into_inner() else {
        return;
    };

    egui::Window::new("SETTINGS")
        .resizable(false)
        .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Sensitivity: ");
                    ui.label("Fov: ");
                });
                ui.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(&mut settings.sensitivity, 0.01..=0.2)
                            .trailing_fill(true)
                            .step_by(0.01),
                    );

                    if ui
                        .add(
                            egui::Slider::new(&mut settings.fov, 5.0..=175.0)
                                .trailing_fill(true)
                                .step_by(5.0),
                        )
                        .changed()
                    {
                        persp.fov = settings.fov / 180.0 * PI;
                    };
                });
            });
            if ui.button("Save").clicked() {
                let serialized = ron::to_string(&settings.sensitivity).unwrap();
                println!("serialized = {}", serialized);
            }
        });
}
