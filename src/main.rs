//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats
mod environment;
mod movable;
mod ownable;
mod player_controller;
mod skybox;
mod spawner;

use bevy::prelude::*;

//use bevy::render::render_resource::Texture;
use crate::environment::Environment;
use crate::movable::UnitMovement;
// use crate::movable::{move_units, MoveTarget};
use crate::ownable::{Selectable, SelectionCircle};
use crate::player_controller::PlayerController;
use crate::spawner::Instance_Spawner;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(PlayerController)
        .add_plugin(Environment)
        .add_plugin(UnitMovement)
        .add_plugin(Instance_Spawner)
        .add_plugin(WorldInspectorPlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        // .add_system(update_emissiveness)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    loaded_units: Query<(Entity, &Handle<StandardMaterial>, &Name)>,
) {
    let texture_handle = asset_server.load("textures/selection_texture.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let scene_handle: Handle<Scene> =
        asset_server.load("../assets/3d_models/units/fighter_01.glb#Scene0");
    // let mut scene : Scene = scene_handle.get_field_mut("Scene").unwrap();
    let parent_id = commands
        .spawn(
            //
            (
                // Transform::from_xyz(0.0, f32::MAX, 0.0).with_scale(Vec3::splat(0.2)),
                SceneBundle {
                    transform: Transform::from_xyz(0.0, 2.0, 0.0).with_scale(Vec3::splat(0.2)),
                    scene: asset_server.load("../assets/3d_models/units/fighter_01.gltf#Scene0"),
                    ..default()
                },
                Selectable {},
                RigidBody::Dynamic,
                Collider::capsule_z(1.0, 1.5),
                GravityScale(0.0),
            ),
        )
        .id();
    let child_id = commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(shape::Plane { size: 5. }.into()),
            material: material_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(SelectionCircle {})
        .id();
    commands.entity(parent_id).push_children(&[child_id]);
}
