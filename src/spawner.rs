use crate::{
    movable::Movable,
    ownable::{Selectable, SelectionCircle},
    player_controller::{Civilisation, RenderLayerMap},
    resources::ResourceType,
    utils::ShapeTypeSerializable,
};
use bevy::{
    prelude::*,
    render::view::RenderLayers,
    utils::{hashbrown::HashMap},
};
use bevy_rapier3d::{prelude::*, rapier::prelude::ShapeType};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
// use std::collections::HashMap;
use std::fmt;
// Create some sort of unit map with regards to civ
impl fmt::Display for Civilisation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Civilisation::Greek => write!(f, "Greek"),
        }
    }
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum UnitStat {
    MaxMiningDist(f32),
    BaseMiningRate(f32),
    BonusMiningRate((ResourceType, f32)),
}
#[derive(Clone, Serialize, Deserialize)]
pub struct UnitStats(pub Vec<UnitStat>);
impl Deref for UnitStats {
    type Target = Vec<UnitStat>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UnitStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum UnitType {
    Cruiser,
    Spacestation,
    MiningStation,
}
impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnitType::Cruiser => write!(f, "Cruiser"),
            UnitType::Spacestation => write!(f, "Space Station"),
            UnitType::MiningStation => write!(f, "Mining Station"),
        }
    }
}

#[derive(Resource)]
pub struct UnitSpecifications {
    pub unit_specifications: HashMap<(Civilisation, UnitType), UnitSpecification>,
}
//TODO specify modifications to model (e.g #update_emissiveness)
#[derive(Clone, Component, Serialize, Deserialize)]
pub struct UnitSpecification {
    pub file_path: String,
    pub scene: String,
    pub icon_path: String,
    pub unit_name: String,
    pub movable: bool,
    pub shape: ShapeTypeSerializable,
    pub dimensions: Vec3,
    pub prescaling: f32,
    pub base_stats: UnitStats,
    pub unit_info: String,
    pub unit_cost: HashMap<ResourceType, f32>,
}
pub struct InstanceSpawner;
#[derive(Event)]
pub struct InstanceSpawnRequest {
    pub location: Vec3,
    pub unit_type: UnitType,
    pub civilisation: Civilisation,
}

#[derive(Component)]
pub struct UnitInformation {
    pub unit_name: String,
    pub unit_type: UnitType,
    pub civilisation: Civilisation,
    pub thumbnail: String,
    pub stats: UnitStats,
    pub unit_info: String,
    pub unit_cost: HashMap<ResourceType, f32>,
}
impl Plugin for InstanceSpawner {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, populate_units)
            .add_systems(Update, spawn)
            .add_event::<InstanceSpawnRequest>();
        // .add_systems(Update, update_emissiveness.before(spawn));
        // populate_units(app);
    }
}
fn populate_units(mut commands: Commands) {
    let mut unit_specifications: UnitSpecifications = UnitSpecifications {
        unit_specifications: HashMap::new(),
    };
    // todo!("Read from file");
    unit_specifications.unit_specifications.insert(
        (Civilisation::Greek, UnitType::Cruiser),
        UnitSpecification {
            file_path: "./assets/3d_models/units/greek/cruiser/greek_cruiser.gltf".into(),
            scene: "Scene0".to_owned(),
            icon_path: "./3d_models/units/greek/cruiser/greek_cruiser_thumbnail.png".into(),
            unit_name: "Andreia Class Cruiser".into(),
            movable: true,
            shape: ShapeTypeSerializable(ShapeType::Capsule),
            dimensions: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 2.0,
            },
            prescaling: 0.1,
            base_stats: UnitStats(Vec::new()),
            unit_info: "The basic cruiser type used by the Greek Empire".into(),
            unit_cost: HashMap::from_iter(vec![(ResourceType::Plotanium, 22.0)]),
        },
    );
    unit_specifications.unit_specifications.insert(
        (Civilisation::Greek, UnitType::MiningStation),
        UnitSpecification {
            file_path: "./assets/3d_models/units/greek/mining_rig/mining_rig.gltf".into(),
            scene: "Scene0".to_owned(),
            icon_path: "./3d_models/units/greek/mining_rig/mining_rig_thumbnail.png".into(),
            unit_name: "Hephaestus Mining Station".into(),
            movable: true,
            shape: ShapeTypeSerializable(ShapeType::Capsule),
            dimensions: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 2.0,
            },
            prescaling: 0.05,
            base_stats: UnitStats(vec![
                UnitStat::MaxMiningDist(1.5),
                UnitStat::BaseMiningRate(24.0),
                UnitStat::BonusMiningRate((ResourceType::Plotanium, 5.0)),
            ]),
            unit_info: "The mining station used by most empires.".into(),
            unit_cost: HashMap::from_iter(vec![(ResourceType::Plotanium, 22.0)]),
        },
    );
    unit_specifications.unit_specifications.insert(
        (Civilisation::Greek, UnitType::Spacestation),
        UnitSpecification {
            file_path: "./assets/3d_models/buildings/greek/spacestation.glb".into(),
            scene: "Scene0".to_owned(),
            icon_path: "./3d_models/buildings/greek/spacestation_thumbnail.png".into(),
            unit_name: "Akinetos Space Station".into(),
            movable: false,
            shape: ShapeTypeSerializable(ShapeType::Ball),
            dimensions: Vec3 {
                x: 50.0,
                y: 50.0,
                z: 30.0,
            },
            prescaling: 0.02,
            base_stats: UnitStats(Vec::new()),
            unit_info: "A mighty spacestation, used to construct ships and defend systems".into(),
            unit_cost: HashMap::from_iter(vec![(ResourceType::Plotanium, 22.0)]),
        },
    );

    commands.insert_resource(unit_specifications);
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
    for spawn_request in spawn_requests.read() {
        if let Some(unit_specification) = unit_specifications
            .unit_specifications
            .get(&(spawn_request.civilisation, spawn_request.unit_type.clone()))
        {
            let texture_handle = asset_server.load("textures/selection_texture.png");
            let material_handle = materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle),
                alpha_mode: AlphaMode::Blend,
                ..default()
            });
            let collider: Collider = match unit_specification.shape.0 {
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
                        .with_scale(Vec3::splat(unit_specification.prescaling)),
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
                        unit_type: spawn_request.unit_type.clone(),
                        civilisation: spawn_request.civilisation,
                        thumbnail: unit_specification.icon_path.clone(),
                        stats: unit_specification.base_stats.clone(),
                        unit_info: unit_specification.unit_info.clone(),
                        unit_cost: unit_specification.unit_cost.clone(),
                    },
                    RigidBody::KinematicPositionBased,
                    collider,
                    GravityScale(0.0),
                    RenderLayers::layer(RenderLayerMap::Main as usize),
                    // ContextMenuActions {},
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(Plane3d::default().mesh().size(
                                2.5 * unit_specification.dimensions.max_element(),
                                2.5 * unit_specification.dimensions.max_element(),
                            )),
                            material: material_handle,
                            transform: Transform::from_scale(Vec3::splat(1.0)),
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        SelectionCircle,
                        RenderLayers::layer(RenderLayerMap::Main as usize),
                    ));
                    parent.spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgba(0.0, 1.0, 0.0, 0.5),
                                ..Default::default()
                            }),
                            ..default()
                        },
                        RenderLayers::layer(RenderLayerMap::Minimap as usize),
                    ));
                })
                .id();

            if unit_specification.movable {
                commands.entity(parent_id).insert(Movable {});
            }
        }
        // commands.entity(entity).remove::<InstanceSpawnRequest>();
    }
}
