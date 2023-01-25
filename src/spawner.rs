use bevy::prelude::*;

// Create some sort of unit map with regards to civ

pub struct Instance_Spawner;

pub struct InstanceSpawnRequest {}

pub struct Custom_Material_Information {
    emissiveness: f32,
}

impl Plugin for Instance_Spawner {
    fn build(&self, app: &mut App) {
        app.add_system(update_emissiveness)
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
            println!("{:?}", glow_material.emissive);
            // Can multiply by factor to reach correct emmisiveness
            glow_material.emissive = Color::rgb(0.0, 20.0, 0.0);

            // if let Some(image_handle) = glow_material.emissive_texture.clone() {
            //     image_assets.get_mut(&image_handle).unwrap().data;
            // }
            println!("Name: {}", name.as_str());
        }
    }
}
