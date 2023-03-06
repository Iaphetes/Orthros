use bevy::{ecs::query::WorldQuery, prelude::*};
use std::collections::HashMap;

// Create some sort of unit map with regards to civ
#[derive(Eq, Hash, PartialEq)]
pub enum Civilisation {
    GREEK,
    ROMAN,
    JAPANESE,
}
#[derive(Eq, Hash, PartialEq)]
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
    location: Vec3,
    unit_type: UnitType,
    civilisation: Civilisation,
}

pub struct Custom_Material_Information {
    emissiveness: f32,
}

impl Plugin for InstanceSpawner {
    fn build(&self, app: &mut App) {
        app.add_system(update_emissiveness);
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
fn spawn(spawn_requests: Query<&InstanceSpawnRequest>, mut commands: Commands) {}

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
