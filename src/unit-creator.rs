mod a_star;
mod civilisation;
mod environment;
mod movable;
mod ownable;
mod player_controller;
mod resource_collection;
mod resources;
mod spawner;
mod ui;
mod utils;

use std::{
    f32::consts::TAU,
    // fs,
    // io::BufWriter
};

use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::{
    color::palettes::css::{DARK_GRAY, DARK_OLIVEGREEN},
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    math::Vec3,
    // prelude::*,
    // reflect::DynamicTypePath,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use bevy_file_dialog::prelude::*;
use bevy_lunex::prelude::*;
// use bevy_rapier3d::parry::shape::ShapeType;
// use resources::ResourceType;
// use spawner::{UnitSpecification, UnitStats};
// use utils::ShapeTypeSerializable;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

struct TextFileContents;
#[derive(Component)]
struct RenderedUnit(String);
struct PrintFilePath;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            FileDialogPlugin::new()
                // allow saving of files marked with TextFileContents
                // allow loading of files marked with TextFileContents
                .with_pick_file::<PrintFilePath>(),
            UiPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // button_system,
                file_picked,
                text_update_system,
                rotate_rendered,
            ),
        )
        .run();
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;
// fn initialise_mini_map(commands: &mut Commands, mut images: ResMut<Assets<Image>>) -> Entity {
//     let size = Extent3d {
//         width: 1024,
//         height: 1024,
//         ..default()
//     };
//     let mut image = Image {
//         texture_descriptor: TextureDescriptor {
//             label: None,
//             size,
//             dimension: TextureDimension::D2,
//             format: TextureFormat::Bgra8UnormSrgb,
//             mip_level_count: 1,
//             sample_count: 1,
//             usage: TextureUsages::TEXTURE_BINDING
//                 | TextureUsages::COPY_DST
//                 | TextureUsages::RENDER_ATTACHMENT,
//             view_formats: &[],
//         },
//         ..default()
//     };
//     image.resize(size);
//     let image_handle = images.add(image);

// commands.spawn((
//     Camera3dBundle {
//         camera_3d: Camera3d { ..default() },
//         camera: Camera {
//             clear_color: ClearColorConfig::Custom(DARK_GRAY.into()),
//             // render before the "main pass" camera
//             order: -1,
//             target: RenderTarget::Image(image_handle.clone()),
//             hdr: true,
//             ..default()
//         },
//         tonemapping: Tonemapping::BlenderFilmic,
//         transform: Transform::from_translation(Vec3::new(10.0, 2.0, 0.0))
//             .looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     },
//     BloomSettings::default(),
//     // RenderLayers::from_layers(&[1]),
// ));
// }
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // UI camera
    commands.spawn((
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        },
    ));

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Create a texture resource that our 3D camera will render to
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // Create the texture
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Initiate the image
    image.resize(size);

    // Add our texture to asset server and get a handle
    let render_image = asset_server.add(image);

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d { ..default() },
            camera: Camera {
                clear_color: ClearColorConfig::Custom(DARK_GRAY.into()),
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(render_image.clone()),
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::BlenderFilmic,
            transform: Transform::from_translation(Vec3::new(10.0, 2.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        // RenderLayers::from_layers(&[1]),
    ));
    commands
        .spawn((
            // This makes the UI entity able to receive camera data
            MovableByCamera,
            // This is our UI system
            UiTreeBundle::<MainUi>::from(UiTree::new2d("Main Menu")),
        ))
        .with_children(|ui| {
            // Spawn boundary node

            let root = UiLink::<MainUi>::path("Root");
            ui.spawn((
                root.clone(),
                // Link the entity
                // Specify UI layout
                UiLayout::window_full().pack::<Base>(),
            ));
            ui.spawn((
                root.add("UnitViewer"),
                UiLayout::window()
                    .size(Rw(40.0))
                    // .size((256.0, 256.0))
                    .anchor(Anchor::CenterLeft)
                    .pos((Rh(0.0), Rh(50.0)))
                    // .align_x(Align::LEFT)
                    // .scaling(Scaling::Fill)
                    .pack::<Base>(),
                UiImage2dBundle::from(render_image),
                PickingPortal, // You can add this component to send picking events through the viewport.
            ));
            let inputs = root.add("Inputs");
            ui.spawn((
                // Link the entity
                // root.add("Inputs"),
                inputs.clone(),
                // Specify UI layout
                UiLayout::window()
                    .size((Rw(50.0), Rh(80.0)))
                    .anchor(Anchor::CenterLeft)
                    .pos((Rl(45.0), Rl(50.0)))
                    .pack::<Base>(),
                // Add image to the entity
                // UiImage2dBundle::from(asset_server.load("textures/ui/greek/button_01.png")),
            ));
            let gltf = inputs.add("GLTF");

            ui.spawn((
                // Link the entity
                // root.add("Inputs"),
                gltf.clone(),
                // Specify UI layout
                UiLayout::window()
                    .size((Rw(80.0), Rh(5.0)))
                    .anchor(Anchor::CenterLeft)
                    .pos((Rl(10.0), Rl(10.0)))
                    .pack::<Base>(),
                // Add image to the entity
                // UiImage2dBundle::from(asset_server.load("textures/ui/greek/button_01.png")),
            ));
            ui.spawn((
                // Link the entity
                // root.add("Inputs"),
                gltf.add("text"),
                // Specify UI layout
                UiLayout::window()
                    .size((Rw(80.0), Rh(100.0)))
                    .anchor(Anchor::CenterLeft)
                    .pos((Rl(10.0), Rl(50.0)))
                    .pack::<Base>(),
                // Add image to the entity
                // UiImage2dBundle::from(asset_server.load("textures/ui/greek/button_01.png")),
            ));
            ui.spawn((
                // Link the entity
                // root.add("Inputs"),
                gltf.add("button"),
                // Specify UI layout
                UiLayout::window()
                    .size((Rw(10.0), Rh(70.0)))
                    .anchor(Anchor::CenterLeft)
                    .pos((Rl(90.0), Rl(50.0)))
                    .pack::<Base>(),
                // Add image to the entity
                UiImage2dBundle::from(asset_server.load("textures/ui/greek/button_01.png")),
            ));
        });
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(
                GltfAssetLabel::Scene(0)
                    .from_asset("3d_models/units/greek/cruiser/greek_cruiser.gltf"),
            ),
            ..default()
        },
        RenderedUnit("3d_models/units/greek/cruiser/greek_cruiser.gltf".to_owned()),
        RenderLayers::layer(1),
    ));
}
// fn button_system(
//     mut commands: Commands,
//     mut interaction_query: Query<
//         (
//             &Interaction,
//             &mut BackgroundColor,
//             &mut BorderColor,
//             &Children,
//         ),
//         (Changed<Interaction>, With<Button>),
//     >,
// ) {
//     for (interaction, mut color, mut border_color, children) in &mut interaction_query {
//         match *interaction {
//             Interaction::Pressed => {
//                 // commands.dialog().load_file::<TextFileContents>();
//                 commands.dialog().pick_file_path::<PrintFilePath>();
//                 *color = PRESSED_BUTTON.into();
//                 border_color.0 = RED.into();
//             }
//             Interaction::Hovered => {
//                 *color = HOVERED_BUTTON.into();
//                 border_color.0 = Color::WHITE;
//             }
//             Interaction::None => {
//                 *color = NORMAL_BUTTON.into();
//                 border_color.0 = Color::BLACK;
//             }
//         }
//     }
// }

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
// This system will rotate any entity in the scene with a Rotatable component around its y-axis.
fn rotate_rendered(mut cameras: Query<&mut Transform, With<RenderedUnit>>, timer: Res<Time>) {
    for mut transform in &mut cameras {
        // The speed is first multiplied by TAU which is a full rotation (360deg) in radians,
        // and then multiplied by delta_seconds which is the time that passed last frame.
        // In other words. Speed is equal to the amount of rotations per second.
        transform.rotate_y(0.3 * TAU * timer.delta_seconds());
    }
}

fn pick(mut commands: Commands) {
    commands.dialog().pick_file_path::<PrintFilePath>();
}

fn file_picked(mut ev_picked: EventReader<DialogFilePicked<PrintFilePath>>) {
    for ev in ev_picked.read() {
        eprintln!("File picked, path {:?}", ev.path);
    }
}
// fn main() {
//     let test_instance = UnitSpecification {
//         file_path: "./assets/3d_models/buildings/greek/spacestation.glb".into(),
//         scene: "Scene0".to_owned(),
//         icon_path: "./3d_models/buildings/greek/spacestation_thumbnail.png".into(),
//         unit_name: "Akinetos Space Station".into(),
//         movable: false,
//         shape: ShapeTypeSerializable(ShapeType::Ball),
//         dimensions: Vec3 {
//             x: 50.0,
//             y: 50.0,
//             z: 30.0,
//         },
//         prescaling: 0.02,
//         base_stats: UnitStats(Vec::new()),
//         unit_info: "The greek Akinetos Space Station. This is the hub of all activity in a system."
//             .into(),
//         unit_cost: vec![(ResourceType::Plotanium, 100.0)].into_iter().collect(),
//     };
//     let serialized: String =
//         ron::ser::to_string_pretty(&test_instance, ron::ser::PrettyConfig::default()).unwrap();
//     let file = fs::File::create("greek_spacestation.ron").expect("Could not create a file");

//     // ron::ser::to_writer_pretty(
//     //     BufWriter::new(file),
//     //     &test_instance,
//     //     ron::ser::PrettyConfig::default(),
//     // );
//     println!("{}", serialized.clone(),);
// }
