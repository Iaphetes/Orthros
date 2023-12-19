#![feature(let_chains)]
mod a_star;
mod civilisation;
mod environment;
mod movable;
mod ownable;
mod player_controller;
mod resource_collection;
mod resources;
mod spawner;
mod ui;

use crate::environment::Environment;
use crate::movable::UnitMovement;
use crate::player_controller::PlayerController;
use crate::spawner::InstanceSpawner;
use crate::ui::GameUI;
use bevy::{
    prelude::*,
    utils::HashMap,
    window::{PresentMode, WindowMode, WindowPlugin, WindowResolution},
};
use bevy_rapier3d::prelude::*;
use civilisation::CivilisationPlugin;
use resource_collection::ResourceCollection;
use resources::{ResourceLevel, ResourceLevels, ResourceType, ResourceUpdateEvent};
use spawner::{Civilisation, InstanceSpawnRequest, UnitType};
enum TechLevel {
    L0,
}

// #[derive(Component)]
// struct ContextMenuActions {
//     actions: Vec<ContextMenuAction>,
// }
#[derive(Component, Clone)]
enum ContextMenuAction {
    Build(UnitType),
}
#[derive(Component)]
struct PlayerInfo {
    civilisation: Civilisation,
    tech_level: TechLevel,
    context_menu_actions: HashMap<UnitType, Vec<ContextMenuAction>>,
}
#[derive(Component)]
struct ActivePlayer;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                resolution: WindowResolution::default().with_scale_factor_override(1.0),
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins((RapierPhysicsPlugin::<NoUserData>::default(), CivilisationPlugin, RapierDebugRenderPlugin::default()))
        .insert_resource(Msaa::Sample4)
        .add_plugins((
            PlayerController,
            Environment,
            UnitMovement,
            InstanceSpawner,
            GameUI,
            ResourceCollection,
            RapierPhysicsPlugin::<NoUserData>::default(),
            CivilisationPlugin,
            RapierDebugRenderPlugin::default(),
        ))
        .add_event::<InstanceSpawnRequest>()
        .add_event::<ResourceUpdateEvent>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut spawn_events: EventWriter<InstanceSpawnRequest>,
    mut resource_update_events: EventWriter<ResourceUpdateEvent>,
) {
    let mut player_info: PlayerInfo = PlayerInfo {
        civilisation: Civilisation::Greek,
        tech_level: TechLevel::L0,
        context_menu_actions: HashMap::new(),
    };

    player_info.context_menu_actions.insert(
        UnitType::Spacestation,
        vec![
            ContextMenuAction::Build(UnitType::Cruiser),
            ContextMenuAction::Build(UnitType::MiningStation),
        ],
    );
    commands.spawn((
        ActivePlayer,
        player_info,
        ResourceLevels(HashMap::from([(ResourceType::Plotanium, 0)])),
    ));
    for x in 0..2 {
        for y in 0..2 {
            spawn_events.send(InstanceSpawnRequest {
                location: Vec3 {
                    x: x as f32 * 2.0,
                    y: 2.0,
                    z: y as f32 * 2.0,
                },
                unit_type: UnitType::Cruiser,
                civilisation: Civilisation::Greek,
            });
        }
    }
    spawn_events.send(InstanceSpawnRequest {
        location: Vec3 {
            x: -3.0,
            y: 2.0,
            z: -3.0,
        },
        unit_type: UnitType::Spacestation,
        civilisation: Civilisation::Greek,
    });
    resource_update_events.send(ResourceUpdateEvent(ResourceLevel {
        resource_type: resources::ResourceType::Plotanium,
        amount: 420,
    }));
}
