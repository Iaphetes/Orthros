#![feature(let_chains)]
mod environment;
mod movable;
mod ownable;
mod player_controller;
mod skybox;
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
    window::{PresentMode, WindowMode, WindowPlugin},
};
use bevy_rapier3d::prelude::*;
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
    BUILD(UnitType),
}
#[derive(Resource)]
struct PlayerInfo {
    civilisation: Civilisation,
    tech_level: TechLevel,

    context_menu_actions: HashMap<UnitType, Vec<ContextMenuAction>>,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(Msaa::Sample4)
        .add_plugin(PlayerController)
        .add_plugin(Environment)
        .add_plugin(UnitMovement)
        .add_plugin(InstanceSpawner)
        .add_plugin(GameUI)
        .add_event::<InstanceSpawnRequest>()
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut spawn_events: EventWriter<InstanceSpawnRequest>) {
    let mut player_info: PlayerInfo = PlayerInfo {
        civilisation: Civilisation::GREEK,
        tech_level: TechLevel::L0,
        context_menu_actions: HashMap::new(),
    };
    player_info.context_menu_actions.insert(
        UnitType::SPACESTATION,
        vec![ContextMenuAction::BUILD(UnitType::CRUISER)],
    );
    commands.insert_resource(player_info);
    for x in 0..2 {
        for y in 0..2 {
            spawn_events.send(InstanceSpawnRequest {
                location: Vec3 {
                    x: x as f32 * 2.0,
                    y: 2.0,
                    z: y as f32 * 2.0,
                },
                unit_type: UnitType::CRUISER,
                civilisation: Civilisation::GREEK,
            });
        }
    }
    spawn_events.send(InstanceSpawnRequest {
        location: Vec3 {
            x: -3.0,
            y: 2.0,
            z: -3.0,
        },
        unit_type: UnitType::SPACESTATION,
        civilisation: Civilisation::GREEK,
    });
}
