use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::geometry::Sensor;
use noise::{NoiseFn, SuperSimplex};

use crate::skybox::Skybox;
use bevy_rapier3d::prelude::*;

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
            .insert_resource(GridSettings {
                cell_size: 0.2,
                grid_width: 1000,
                grid_height: 1000,
                x_y_offset: Vec2::new(1.0, 1.0),
                density: 0.2,
            });
    }
}
// fn generate_obstacles(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     grid_settings: Res<GridSettings>,
//     mut gridmap_q: Query<&mut MovementGrid>,
// ) {
//     let noise_generator: SuperSimplex = SuperSimplex::new(SuperSimplex::DEFAULT_SEED);
//     //    let mut gridmap : &MovementGrid;
//     match gridmap_q.get_single_mut() {
//         Ok(mut gridmap) => {
//             for i in 0..grid_settings.grid_width as usize {
//                 for j in 0..grid_settings.grid_height as usize {
//                     if noise_generator.get([i as f64, j as f64]) > grid_settings.density {
//                         gridmap.grid[i][j] = 1;
//                     }
//                 }
//             }
//         }
//         Err(error) => {
//             println!("{:?}", error);
//             return;
//         }
//     }
// }

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
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
    pub x_y_offset: Vec2,
    pub density: f64, // TODO put into map generation
}
#[derive(Component)]
pub struct MovementGrid {
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
fn setup_movement_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_settings: Res<GridSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut gridmap: MovementGrid = MovementGrid { grid: Vec::new() };
    //    commands.spawn().insert(MovementGrid{
    //        grid: Vec::new()
    //    });
    for i in 0..grid_settings.grid_width as usize {
        gridmap.grid.push(Vec::new());
        for j in 0..grid_settings.grid_height as usize {
            gridmap.grid[i].push(0);

            // commands.spawn(MaterialMeshBundle {
            //     mesh: meshes.add(
            //         shape::Plane {
            //             size: grid_settings.cell_size,
            //         }
            //         .into(),
            //     ),

            //     transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)).with_translation(
            //         Vec3::new(
            //             i as f32 * grid_settings.cell_size - grid_settings.x_y_offset.x,
            //             0.0,
            //             j as f32 * grid_settings.cell_size - grid_settings.x_y_offset.y,
            //         ),
            //     ),
            //     material: standard_materials.add(StandardMaterial {
            //         base_color_texture: Some(asset_server.load("textures/bloody_rectangle.png")),
            //         alpha_mode: AlphaMode::Blend,
            //         ..default()
            //     }),
            //     ..default()
            // });
        }
    }
    println!("spawning gridmap");
    commands.spawn(gridmap);
}
