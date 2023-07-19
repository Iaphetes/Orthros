use crate::{
    movable::Movable,
    ownable::{Selectable, SelectionCircle},
    player_controller::RenderLayerMap,
};
use bevy::{prelude::*, render::view::RenderLayers, scene::SceneInstance};
use bevy_rapier3d::{prelude::*, rapier::prelude::ShapeType};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
// Create some sort of unit map with regards to civ
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Civilisation {
    Greek,
    // ROMAN,
    // JAPANESE,
}
impl fmt::Display for Civilisation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Civilisation::Greek => write!(f, "Greek"),
        }
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum UnitType {
    Cruiser,
    Spacestation,
}
impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitType::Cruiser => write!(f, "Cruiser"),

            UnitType::Spacestation => write!(f, "Space Station"),
        }
    }
}

#[derive(Resource)]
pub struct UnitSpecifications {
    pub unit_specifications: HashMap<(Civilisation, UnitType), UnitSpecification>,
}
//TODO specify modifications to model (e.g #update_emissiveness)
#[derive(Clone, Component)]
pub struct UnitSpecification {
    pub file_path: String,
    pub scene: String,
    pub icon_path: String,
    pub unit_name: String,
    pub movable: bool,
    pub shape: ShapeType,
    pub dimensions: Vec3,
    pub _prescaling: f32,
}
pub struct InstanceSpawner;
#[derive(Event)]
pub struct InstanceSpawnRequest {
    pub location: Vec3,
    pub unit_type: UnitType,
    pub civilisation: Civilisation,
}

pub struct _CustomMaterialInformation {
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
        app.add_systems(Update, spawn)
            .add_event::<InstanceSpawnRequest>()
            .add_systems(Update, update_emissiveness.before(spawn));
        populate_units(app);
    }
}
fn populate_units(app: &mut App) {
    let mut unit_specifications: UnitSpecifications = UnitSpecifications {
        unit_specifications: HashMap::new(),
    };
    unit_specifications.unit_specifications.insert(
        (Civilisation::Greek, UnitType::Cruiser),
        UnitSpecification {
            file_path: "./assets/3d_models/units/greek/fighter_01.gltf".into(),
            scene: "Scene0".to_owned(),
            icon_path: "./3d_models/units/greek/greek_cruiser_thumbnail.png".into(),
            unit_name: "Andreia Class Cruiser".into(),
            movable: true,
            shape: ShapeType::Capsule,
            dimensions: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 2.0,
            },
            _prescaling: 0.2,
        },
    );
    unit_specifications.unit_specifications.insert(
        (Civilisation::Greek, UnitType::Spacestation),
        UnitSpecification {
            file_path: "./assets/3d_models/buildings/greek/spacestation.gltf".into(),
            scene: "Scene0".to_owned(),
            icon_path: "./3d_models/buildings/greek/spacestation_thumbnail.png".into(),
            unit_name: "Akinetos Space Station".into(),
            movable: false,
            shape: ShapeType::Capsule,
            dimensions: Vec3 {
                x: 10.0,
                y: 15.0,
                z: 10.0,
            },
            _prescaling: 0.2,
        },
    );

    app.insert_resource(unit_specifications);
}
#[derive(Component)]
pub struct EntityWrapper {
    pub entity: Entity,
}
fn spawn(
    mut spawn_requests: EventReader<InstanceSpawnRequest>,
    mut commands: Commands,
    unit_specifications: Res<UnitSpecifications>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for spawn_request in spawn_requests.iter() {
        if let Some(unit_specification) = unit_specifications
            .unit_specifications
            .get(&(spawn_request.civilisation, spawn_request.unit_type))
        {
            let texture_handle = asset_server.load("textures/selection_texture.png");
            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle),
                alpha_mode: AlphaMode::Blend,
                ..default()
            });
            let collider: Collider = match unit_specification.shape {
                ShapeType::Ball => Collider::ball(unit_specification.dimensions.max_element()),
                ShapeType::Capsule => Collider::capsule_z(
                    unit_specification.dimensions.max_element() / 2.0,
                    unit_specification.dimensions.min_element(),
                ),
                shape => {
                    println!("Shape {:?} not supported", shape);
                    continue;
                }
            };
            let parent_id = commands
                .spawn((
                    SceneBundle {
                        transform: Transform::from_xyz(
                            spawn_request.location.x,
                            spawn_request.location.y,
                            spawn_request.location.z,
                        )
                        .with_scale(Vec3::splat(0.10)),
                        scene: asset_server.load(
                            unit_specification
                                .file_path
                                .clone()
                                .replace("./assets/", "")
                                + "#"
                                + &unit_specification.scene,
                        ),
                        ..default()
                    },
                    Selectable {},
                    UnitInformation {
                        unit_name: unit_specification.unit_name.clone(),
                        unit_type: spawn_request.unit_type,
                        civilisation: spawn_request.civilisation,
                        thumbnail: unit_specification.icon_path.clone(),
                    },
                    RigidBody::KinematicPositionBased,
                    collider,
                    GravityScale(0.0),
                    RenderLayers::layer(RenderLayerMap::Main as u8),
                    // ContextMenuActions {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MaterialMeshBundle {
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
                        },
                        SelectionCircle,
                        RenderLayers::layer(RenderLayerMap::Main as u8),
                    ));
                    parent.spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(
                                shape::Plane {
                                    size: 10.0,
                                    subdivisions: 1,
                                }
                                .into(),
                            ),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgba(0.0, 1.0, 0.0, 0.5),
                                ..Default::default()
                            }),
                            ..default()
                        },
                        RenderLayers::layer(RenderLayerMap::Minimap as u8),
                    ));
                })
                .id();

            commands.spawn((
                EntityWrapper { entity: parent_id },
                (*unit_specification).clone(),
            ));
            if unit_specification.movable {
                commands.entity(parent_id).insert(Movable {});
            }
        }
        // commands.entity(entity).remove::<InstanceSpawnRequest>();
    }
}
fn read_emissive_color(gltf_material: &Value) -> Option<Color> {
    if let serde_json::Value::Object(map) = &gltf_material["extensions"] {
        for (extension_name, extension) in map {
            if extension_name != "KHR_materials_emissive_strength" {
                continue;
            }
            if let serde_json::Value::Number(num) = &extension["emissiveStrength"] {
                let emissiveness = num.as_u64()? * 10;
                return gltf_material["emissiveFactor"]
                    .as_array()
                    .as_ref()
                    .map(|emissive_vals| {
                        Color::rgb_linear(
                            emissive_vals[0].as_f64().unwrap() as f32 * emissiveness as f32,
                            emissive_vals[1].as_f64().unwrap() as f32 * emissiveness as f32,
                            emissive_vals[2].as_f64().unwrap() as f32 * emissiveness as f32,
                        )
                    });
            }
        }
    }
    None
}

fn update_emissiveness(
    mut commands: Commands,
    unit_info: Query<(Entity, &EntityWrapper, &UnitSpecification)>,
    mut material_handles: Query<(&Name, &mut Handle<StandardMaterial>)>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    scene_instances: Query<&SceneInstance>,
    scene_spawner: Res<SceneSpawner>,
) {
    for (entity, entity_wrapper, info) in unit_info.iter() {
        println!("{}", info.file_path);
        let mut file = File::open(&info.file_path).unwrap();

        let mut contents: String = String::new();
        if let Err(error) = file.read_to_string(&mut contents) {
            debug!("Could not read file content to string due to: {:?}", error);
            continue;
        }
        let gltf_model: serde_json::Value = serde_json::from_str(&contents).unwrap();
        let mut material_nr = 0;
        let mut updated = true;
        if let Some(material_str) = gltf_model["materials"].as_array() {
            for material in material_str {
                // println!("{:?}", material);
                let meshes: Vec<Value> = match gltf_model["meshes"].as_array() {
                    Some(meshes) => meshes.to_owned(),
                    None => Vec::new(),
                };
                if material_nr >= meshes.len() {
                    material_nr += 1;
                    continue;
                }
                let mesh_name: String = meshes[material_nr]["name"].to_string();
                if let Some(emissive_color) = read_emissive_color(material) {
                    updated = false;
                    match scene_instances.get(entity_wrapper.entity) {
                        Ok(scene_instance) => {
                            for scene_entity in
                                scene_spawner.iter_instance_entities(**scene_instance.to_owned())
                            {
                                match material_handles.get_mut(scene_entity) {
                                    Ok((name, mut material_handle)) => {
                                        if name.to_string() == mesh_name.trim_matches('"') {
                                            match material_assets.get_mut(&material_handle) {
                                                Some(material) => {
                                                    println!(
                                                        "Old material{:#?}",
                                                        material.emissive
                                                    );

                                                    let mut new_material: StandardMaterial =
                                                        material.clone();

                                                    new_material.emissive = emissive_color;
                                                    *material_handle =
                                                        material_assets.add(new_material);

                                                    updated = true;
                                                }
                                                None => {
                                                    println!("Invalid shader handle")
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        println!("No material attached to entity {:?}", error)
                                    }
                                }
                            }
                        }
                        Err(error) => println!("No scene attached to entity {:?}", error),
                    }
                }
                material_nr += 1;
            }
        }
        if updated {
            commands.entity(entity).despawn_recursive();
        }
    }
}
