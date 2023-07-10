use crate::player_controller::RenderLayerMap;
use crate::skybox::Skybox;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
};
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::geometry::Sensor;

pub struct Environment;

impl Plugin for Environment {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<CustomMaterial>::default())
            .add_plugin(Skybox)
            .add_startup_system(environment_setup)
            .add_startup_system(setup_movement_grid)
            // .add_startup_system_to_stage(
            //     bevy::app::StartupStage::PostStartup,
            //     generate_obstacles.after(setup_movement_grid),
            // )
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
                color: Color::GREEN,
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

    // ambient light
    // NOTE: The ambient light is used to scale how bright the environment map is so with a bright
    // environment map, use an appropriate colour and brightness to match
    commands.insert_resource(AmbientLight {
        color: Color::rgba(1.0, 1.0, 1.0, 1.0),
        brightness: 1.0,
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
