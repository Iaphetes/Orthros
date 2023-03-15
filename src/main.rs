//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats
mod environment;
mod movable;
mod ownable;
mod player_controller;
mod skybox;
mod spawner;

use bevy::prelude::*;
use spawner::{Civilisation, InstanceSpawnRequest, UnitType};

//use bevy::render::render_resource::Texture;
use crate::environment::Environment;
use crate::movable::UnitMovement;
// use crate::movable::{move_units, MoveTarget};
use crate::player_controller::PlayerController;
use crate::spawner::InstanceSpawner;
use bevy_rapier3d::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .insert_resource(Msaa::Sample4)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(PlayerController)
        .add_plugin(Environment)
        // .add_plugin(UnitMovement)
        .add_plugin(InstanceSpawner)
        // .add_plugin(WorldInspectorPlugin/* ) */
        .add_startup_system(setup)
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,

            ..default()
        })
        .run();
}

fn setup(mut commands: Commands) {
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn(InstanceSpawnRequest {
                location: Vec3 {
                    x: x as f32,
                    y: 2.0,
                    z: y as f32,
                },
                unit_type: UnitType::CRUISER,
                civilisation: Civilisation::GREEK,
            });
        }
    }
    commands.spawn(InstanceSpawnRequest {
        location: Vec3 {
            x: -3.0,
            y: 2.0,
            z: -3.0,
        },
        unit_type: UnitType::SPACESTATION,
        civilisation: Civilisation::GREEK,
    });
}
