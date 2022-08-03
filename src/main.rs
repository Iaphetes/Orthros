//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats

mod camera_controller;
mod skybox;
mod environment;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::camera_controller::{camera_controller, CameraControl, CameraControllerSettings};
use crate::environment::Environment;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Environment)
        .add_plugin(CameraControl)
        .add_startup_system(setup)
        .run();
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {





    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("3d_models/blade_starship/scene.gltf#Scene0"),
        transform: Transform::from_scale(Vec3::new(0.2, 0.2, 0.2)),
        ..default()
    });
}
