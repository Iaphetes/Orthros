//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats
mod camera_controller;
mod skybox;
mod environment;
use bevy::{
    prelude::*
};
use bevy::render::render_resource::Texture;

use crate::camera_controller::{CameraControl};
use crate::environment::Environment;
use crate::shape::Plane;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Environment)
        .add_plugin(CameraControl)
        .add_startup_system(setup)
        // .add_system(deselect_test)
        .run();
}
#[derive(Component)]
struct Selectable{
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let texture_handle = asset_server.load("textures/selection_texture.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands.spawn().insert(Transform::from_scale(Vec3::splat(0.2)))
        .insert_bundle(SceneBundle {
        scene: asset_server.load("3d_models/blade_starship/scene.gltf#Scene0"),

        ..default()
    }).insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(shape::Plane { size: 5. }.into()),
        material: material_handle,
        transform: Transform::from_scale(Vec3::splat(0.2)),

        ..default()
    })
    .insert(Selectable {});

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("textures/selection_texture.png"),
        ..default()
    });
}

// fn disable_visual_select(material : &mut Handle<StandardMaterial>){
//     material.
// }
// fn deselect_test(
//     mut query: Query<Entity, With<(Selectable, Handle<Mesh>)>>,
// ){
//     for  entity in query.iter_mut(){
//         entity.remove
//
//     }
// }