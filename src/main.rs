//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats

mod skybox;
mod camera_controller;
use bevy::{
    prelude::*,
};

use crate::skybox::{Skybox};
use crate::camera_controller::{camera_controller, CameraController};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Skybox)
        .add_startup_system(setup)

                .add_system(camera_controller)
        .add_system(animate_light_direction)
        .run();
}
fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}
fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
    // directional 'sun' light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 8.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        })
        .insert(CameraController::default());
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 1. }.into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate colour and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });

    }
