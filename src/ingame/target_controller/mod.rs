use bevy::prelude::*;

pub mod targets;

use targets::*;

pub struct TargetControllerPlugin;

impl Plugin for TargetControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, target_setup)
            .add_systems(
                Update,
                (
                    //target systems
                    hit_detector,
                    circle_target_controller,
                    enemy_target_controller,
                    enemy_target_hostage_controller,
                ),
            )
            //events
            .add_message::<CircleTargetMessage>();
    }
}
