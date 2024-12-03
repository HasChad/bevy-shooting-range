use bevy::prelude::*;
use std::time::Duration;

use super::{WeaponReloadingEvent, WeaponShootingEvent, WeaponState};

#[derive(Resource)]
pub struct P226Animations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Resource)]
pub struct AK15Animations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

pub fn p226_animation_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load("models/weapons/P226.glb#Animation0"),
        asset_server.load("models/weapons/P226.glb#Animation1"),
    ]);

    commands.insert_resource(P226Animations {
        animations: node_indices,
        graph: graphs.add(graph),
    });
}

pub fn ak15_animation_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load("models/weapons/AK15.glb#Animation0"),
        asset_server.load("models/weapons/AK15.glb#Animation1"),
    ]);

    commands.insert_resource(AK15Animations {
        animations: node_indices,
        graph: graphs.add(graph),
    });
}

pub fn weapon_animation_setup(
    mut commands: Commands,
    weapon_state: Res<State<WeaponState>>,
    p226_anim: Res<P226Animations>,
    ak15_anim: Res<AK15Animations>,
    mut players: Query<Entity, Added<AnimationPlayer>>,
) {
    for entity in &mut players {
        let animation_graph = match weapon_state.get() {
            WeaponState::P226 => p226_anim.graph.clone(),
            WeaponState::AK15 => ak15_anim.graph.clone(),
        };

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animation_graph))
            .insert(AnimationTransitions::new());
    }
}

pub fn weapon_animation_player(
    mut shot_event_reader: EventReader<WeaponShootingEvent>,
    mut reload_event_reader: EventReader<WeaponReloadingEvent>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for (mut player, mut transitions) in &mut animation_players {
        for _event in shot_event_reader.read() {
            transitions.play(&mut player, AnimationNodeIndex::new(1), Duration::ZERO);
        }

        for _event in reload_event_reader.read() {
            transitions.play(&mut player, AnimationNodeIndex::new(2), Duration::ZERO);
        }
    }
}
