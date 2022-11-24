use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::geometry::Sensor;

use crate::skybox::Skybox;
use bevy_rapier3d::prelude::*;

pub struct Environment;

impl Plugin for Environment {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<CustomMaterial>::default())
            .add_plugin(Skybox)
            .add_startup_system(environment_setup);
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}
/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/plane_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub fn environment_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    // directional 'sun' light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
    commands.spawn(
        (MaterialMeshBundle {
            mesh: meshes.add(shape::Plane { size: 200. }.into()),
            material: custom_materials.add(CustomMaterial {
                color: Color::GREEN,
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        }),
    );
    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0),
        Collider::cuboid(100.0, 2.0, 100.0),
        Sensor,
    ));

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate colour and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 1.0,
    });
}
