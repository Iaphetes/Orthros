use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt;

use crate::{
    movable::Movable,
    ownable::{Selectable, SelectionCircle},
};
use bevy_rapier3d::{prelude::*, rapier::prelude::ShapeType};
// Create some sort of unit map with regards to civ
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Civilisation {
    GREEK,
    // ROMAN,
    // JAPANESE,
}
impl fmt::Display for Civilisation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Civilisation::GREEK => write!(f, "Greek"),
        }
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum UnitType {
    CRUISER,
    SPACESTATION,
}
impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitType::CRUISER => write!(f, "Cruiser"),

            UnitType::SPACESTATION => write!(f, "Space Station"),
        }
    }
}

#[derive(Resource)]
pub struct UnitSpecifications {
    unit_specifications: HashMap<(Civilisation, UnitType), UnitSpecification>,
}
//TODO specify modifications to model (e.g #update_emissiveness)
pub struct UnitSpecification {
    file_path: String,
    unit_name: String,
    movable: bool,
    shape: ShapeType,
    dimensions: Vec3,
    prescaling: f32,
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
#[derive(Component)]
pub struct UnitInformation {
    pub unit_name: String,
    pub unit_type: UnitType,
    pub civilisation: Civilisation,
    pub thumbnail: String,
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
            file_path: "../assets/3d_models/units/greek/fighter_01.gltf#Scene0".into(),
            unit_name: "Andreia Class Cruiser".into(),
            movable: true,
            shape: ShapeType::Capsule,
            dimensions: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 2.0,
            },
            prescaling: 0.2,
        },
    );
    unit_specifications.unit_specifications.insert(
        (Civilisation::GREEK, UnitType::SPACESTATION),
        UnitSpecification {
            file_path: "../assets/3d_models/buildings/greek/spacestation.gltf#Scene0".into(),
            unit_name: "Akinetos Space Station".into(),
            movable: false,
            shape: ShapeType::Capsule,
            dimensions: Vec3 {
                x: 10.0,
                y: 15.0,
                z: 10.0,
            },
            prescaling: 0.2,
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
                let collider: Collider;
                match unit_specification.shape {
                    ShapeType::Ball => {
                        collider = Collider::ball(unit_specification.dimensions.max_element());
                    }
                    ShapeType::Capsule => {
                        collider = Collider::capsule_z(
                            unit_specification.dimensions.max_element() / 2.0,
                            unit_specification.dimensions.min_element(),
                        );
                    }
                    shape => {
                        println!("Shape {:?} not supported", shape);
                        continue;
                    }
                }
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
                        UnitInformation {
                            unit_name: unit_specification.unit_name.clone(),
                            unit_type: spawn_request.unit_type,
                            civilisation: spawn_request.civilisation,
                            thumbnail: "./3d_models/units/greek/greek_cruiser_thumbnail.png"
                                .into(),
                        },
                        RigidBody::KinematicPositionBased,
                        // MassProperties{

                        //     }
                        collider,
                        GravityScale(0.0),
                    ))
                    .id();
                if unit_specification.movable {
                    commands.entity(parent_id).insert(Movable {});
                }
                let child_id = commands
                    .spawn(MaterialMeshBundle {
                        mesh: meshes.add(
                            shape::Plane {
                                size: 2.5 * unit_specification.dimensions.max_element(),
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
            // Can multiply by factor to reach correct emmisiveness
            glow_material.emissive = Color::rgb_linear(0.0, 250.0, 0.0);

            // if let Some(image_handle) = glow_material.emissive_texture.clone() {
            //     image_assets.get_mut(&image_handle).unwrap().data;
            // }
            // println!("Name: {}", name.as_str());
        }
    }
}
