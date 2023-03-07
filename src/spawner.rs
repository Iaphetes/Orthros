use bevy::{ecs::query::WorldQuery, prelude::*};
use std::collections::HashMap;

use crate::ownable::{Selectable, SelectionCircle};
use bevy_rapier3d::prelude::*;
// Create some sort of unit map with regards to civ
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Civilisation {
    GREEK,
    // ROMAN,
    // JAPANESE,
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum UnitType {
    CRUISER,
}
#[derive(Resource)]
pub struct UnitSpecifications {
    unit_specifications: HashMap<(Civilisation, UnitType), UnitSpecification>,
}
//TODO specify modifications to model (e.g #update_emissiveness)
pub struct UnitSpecification {
    file_path: String,
}
pub struct InstanceSpawner;
#[derive(Component)]
pub struct InstanceSpawnRequest {
    pub location: Vec3,
    pub unit_type: UnitType,
    pub civilisation: Civilisation,
}

pub struct CustomMaterialInformation {
    emissiveness: f32,
}

impl Plugin for InstanceSpawner {
    fn build(&self, app: &mut App) {
        app.add_system(spawn).add_system(update_emissiveness);
        populate_units(app);
    }
}
fn populate_units(app: &mut App) {
    let mut unit_specifications: UnitSpecifications = UnitSpecifications {
        unit_specifications: HashMap::new(),
    };
    unit_specifications.unit_specifications.insert(
        (Civilisation::GREEK, UnitType::CRUISER),
        UnitSpecification {
            file_path: "../assets/3d_models/units/fighter_01.gltf#Scene0".into(),
        },
    );
    app.insert_resource(unit_specifications);
}
fn spawn(
    spawn_requests: Query<(Entity, &InstanceSpawnRequest)>,
    mut commands: Commands,
    unit_specifications: Res<UnitSpecifications>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, spawn_request) in spawn_requests.iter() {
        match unit_specifications
            .unit_specifications
            .get(&(spawn_request.civilisation, spawn_request.unit_type))
        {
            Some(unit_specification) => {
                let texture_handle = asset_server.load("textures/selection_texture.png");
                let material_handle = materials.add(StandardMaterial {
                    base_color_texture: Some(texture_handle),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                });
                let parent_id = commands
                    .spawn((
                        SceneBundle {
                            transform: Transform::from_xyz(
                                spawn_request.location.x,
                                spawn_request.location.y,
                                spawn_request.location.z,
                            )
                            .with_scale(Vec3::splat(0.2)),
                            scene: asset_server.load(&unit_specification.file_path),
                            ..default()
                        },
                        Selectable {},
                        RigidBody::Dynamic,
                        Collider::capsule_z(1.0, 1.5),
                        GravityScale(0.0),
                    ))
                    .id();
                let child_id = commands
                    .spawn(MaterialMeshBundle {
                        mesh: meshes.add(
                            shape::Plane {
                                size: 5.,
                                subdivisions: 1,
                            }
                            .into(),
                        ),
                        material: material_handle,
                        transform: Transform::from_scale(Vec3::splat(1.0)),
                        visibility: Visibility::Hidden,
                        ..default()
                    })
                    .insert(SelectionCircle {})
                    .id();
                commands.entity(parent_id).push_children(&[child_id]);
            }
            None => {}
        }
        commands.entity(entity).remove::<InstanceSpawnRequest>();
    }
}

fn update_emissiveness(
    mut commands: Commands,
    loaded_units: Query<(Entity, &Handle<StandardMaterial>, &Name)>,
    mut mesh_assets: ResMut<Assets<StandardMaterial>>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    for (entity, material_handle, name) in loaded_units.into_iter() {
        if name.as_str() == "Cube.002" {
            let mut glow_material: &mut StandardMaterial =
                mesh_assets.get_mut(material_handle).unwrap();
            // println!("{:?}", glow_material.emissive);
            // Can multiply by factor to reach correct emmisiveness
            glow_material.emissive = Color::rgb(0.0, 50.0, 0.0);

            // if let Some(image_handle) = glow_material.emissive_texture.clone() {
            //     image_assets.get_mut(&image_handle).unwrap().data;
            // }
            // println!("Name: {}", name.as_str());
        }
    }
}
