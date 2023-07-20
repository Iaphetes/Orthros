use crate::skybox::Skybox;
use crate::spawner::UnitSpecification;
use crate::{player_controller::RenderLayerMap, spawner::EntityWrapper};
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
};
use bevy_rapier3d::geometry::Sensor;
use bevy_rapier3d::{
    geometry::Collider,
    prelude::{GravityScale, RigidBody},
};

pub struct Environment;

impl Plugin for Environment {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_plugins(Skybox)
            .add_systems(Startup, (environment_setup, setup_movement_grid))
            .insert_resource(MovementGrid {
                settings: GridSettings {
                    cell_size: 0.2,
                    grid_width: 1000,
                    grid_height: 1000,
                    xy_offset: Vec2::new(500.0, 500.0),
                    density: 0.2,
                },
                grid: Vec::new(),
            });
    }
}
// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, TypePath)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}

#[derive(Resource)]
pub struct GridSettings {
    pub cell_size: f32,
    pub grid_width: u32,
    pub grid_height: u32,
    pub xy_offset: Vec2,
    pub density: f64, // TODO put into map generation
}
#[derive(Resource)]
pub struct MovementGrid {
    pub settings: GridSettings,
    pub grid: Vec<Vec<u8>>,
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
    asset_server: Res<AssetServer>,
) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
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
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(
                shape::Plane {
                    size: 200.,
                    subdivisions: 1,
                }
                .into(),
            ),
            material: custom_materials.add(CustomMaterial {
                color: Color::GRAY,
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        },
        RenderLayers::layer(RenderLayerMap::Main as u8),
    ));
    commands.spawn((
        Transform::from_xyz(0.0, 2.0, 0.0),
        Collider::cuboid(100.0, 2.0, 100.0),
        Sensor,
    ));

    commands.spawn((
        SceneBundle {
            scene: asset_server.load("3d_models/environment/planet.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 2.0, 6772.0)
                .with_rotation(Quat::from_rotation_y((75.0_f32).to_radians())),
            // transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        GravityScale(0.0),
        RenderLayers::layer(RenderLayerMap::Main as u8),
        // ContextMenuActions {},
    ));
    let parent: Entity = commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("3d_models/environment/asteroid_01.gltf#Scene0"),
                transform: Transform::from_xyz(-5.0, 2.0, 5.0), //.with_scale(Vec3::splat(0.001)),
                // transform: Transform::from_scale(Vec3::splat(0.5)),
                ..default()
            },
            RigidBody::KinematicPositionBased,
            GravityScale(0.0),
            RenderLayers::layer(RenderLayerMap::Main as u8),
            // ContextMenuActions {},
        ))
        .id();
    commands.spawn((
        EntityWrapper { entity: parent },
        UnitSpecification {
            file_path: "assets/3d_models/environment/asteroid_01.gltf".to_owned(),
            scene: "Scene0".to_owned(),
            icon_path: "".to_owned(),
            unit_name: "Asteroid".to_owned(),
            movable: true,
            shape: bevy_rapier3d::rapier::prelude::ShapeType::Ball,
            dimensions: Vec3::splat(1.0),
            _prescaling: 1.0,
        },
    ));
    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate colour and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::ANTIQUE_WHITE,
        brightness: 1.0,
    });
    commands.spawn(SpotLightBundle {
        transform: Transform::from_xyz(-1.0, 20.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 1600000.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::DARK_GRAY,
            shadows_enabled: true,
            // inner_angle: 0.6,
            // outer_angle: 0.8,
            ..default()
        },
        ..default()
    });
}
fn setup_movement_grid(mut movement_grid: ResMut<MovementGrid>) {
    for i in 0..movement_grid.settings.grid_width as usize {
        movement_grid.grid.push(Vec::new());
        for _ in 0..movement_grid.settings.grid_height as usize {
            movement_grid.grid[i].push(0);
        }
    }
}
