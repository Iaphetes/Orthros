use crate::ownable::Selectable;
use crate::player_controller::{DeselectEvent, RayHit, RenderLayerMap};
use crate::spawner::UnitInformation;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
#[derive(PartialEq, Eq, Clone, Copy)]
enum UIType {
    MapUI,
    SelectionInfo,
    ContextMenu,
    Diagnostics,
}
#[derive(Component, PartialEq, Eq, Clone, Copy)]
enum UIContent {
    Content(UIType),
    Decoration(UIType),
}
pub struct GameUI;
impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_startup_system(game_overlay)
            .add_event::<RayHit>()
            .add_event::<DeselectEvent>()
            .add_system(change_text_system)
            .add_system(populate_lower_ui)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system(clear_ui.before(populate_lower_ui));
    }
}
fn initialise_mini_map(
    commands: &mut Commands,
    // asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) -> Entity {
    let size = Extent3d {
        width: 1024,
        height: 1024,
        ..default()
    };
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
    image.resize(size);
    let image_handle = images.add(image);
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[RenderLayerMap::General as u8, RenderLayerMap::Minimap as u8]),
    ));
    commands
        .spawn(ImageBundle {
            image: UiImage::from(image_handle),
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                ..Default::default()
            },
            ..default()
        })
        .id()
}
fn create_ui_segment(
    commands: &mut Commands,
    style: Style,
    ui_type: UIType,
    content: Vec<Entity>,
    decoration: Vec<Entity>,
) -> Entity {
    commands
        .spawn((NodeBundle { style, ..default() },))
        .with_children(|parent| {
            parent
                .spawn((
                    UIContent::Content(ui_type),
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            position: UiRect {
                                top: Val::Percent(0.0),
                                left: Val::Px(0.0),
                                ..default()
                            },
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Start,
                            ..default()
                        },
                        background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),

                        ..default()
                    },
                ))
                .push_children(&content);
            parent.spawn((
                UIContent::Decoration(ui_type),
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    // background_color: Color::rgba(0.0, 0.0, 1.0, 0.5).into(),
                    ..default()
                },
            )).push_children(&decoration);
        })
        .id()
}
fn game_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let map_ui_content: Vec<Entity> = vec![initialise_mini_map(&mut commands, images)];
    let map_ui_decoration: Vec<Entity> = vec![commands.spawn(ImageBundle{
        style: Style{
            size: Size{
                height: Val::Percent(100.0),
                width: Val::Percent(100.0) 
            },
            ..default()
        },
        image: UiImage{

            texture: asset_server.load("textures/ui/greek/map_decoration.png"),
            ..default()
        },
        ..default()}).id()];
    let context_menu_decoration: Vec<Entity> = vec![commands.spawn(ImageBundle{
        style: Style{
            size: Size{
                height: Val::Percent(100.0),
                width: Val::Percent(100.0) 
            },
            ..default()
        },
        image: UiImage{

            texture: asset_server.load("textures/ui/greek/context_menu_decoration.png"),
            ..default()
        },
        ..default()}).id()];


    let lower_ui_elements: Vec<Entity> = vec![
        create_ui_segment(
            &mut commands,
            Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                position: UiRect {
                    top: Val::Percent(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            UIType::ContextMenu,
            Vec::new(),
            context_menu_decoration,
        ),
        create_ui_segment(
            &mut commands,
            Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                position: UiRect {
                    top: Val::Percent(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            UIType::SelectionInfo,
            Vec::new(),
            Vec::new(),
        ),
        create_ui_segment(
            &mut commands,
            Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                position: UiRect {
                    top: Val::Percent(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            UIType::MapUI,
            map_ui_content,
            map_ui_decoration,
        ),
    ];
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),

                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // background_color: Color::rgb(1.0, 1.0, 1.0).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(5.0)),

                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        UIContent::Content(UIType::Diagnostics),
                        TextBundle::from_section(
                            format!("FPS - ms/Frame"),
                            TextStyle {
                                font: asset_server
                                    .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                    ));
                });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                        position: UiRect {
                            top: Val::Percent(80.0),
                            left: Val::Px(0.0),
                            ..default()
                        },
                        // align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        flex_wrap: FlexWrap::Wrap,
                        align_content: AlignContent::SpaceBetween,
                        ..default()
                    },
                    // background_color: Color::rgba(1.0, 1.0, 1.0, 0.5).into(),
                    ..default()
                })
                .push_children(&lower_ui_elements);
            // .with_children(|parent| {
            //     parent
            //         .spawn((NodeBundle {
            //             style: Style {
            //                 size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
            //                 ..default()
            //             },

            //             background_color: Color::rgba(0.0, 0.0, 1.0, 0.5).into(),
            //             ..default()
            //         },));

            // });
        });
    // Main lower window
}

fn populate_lower_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ray_hit_event: EventReader<RayHit>,
    mut unit_info: Query<&UnitInformation, With<Selectable>>,
    ui_elements: Query<(Entity, &UIContent)>,
) {
    for hit in ray_hit_event.iter() {
        if hit.mouse_key_enable_mouse {
            let selection_info_content: Entity = ui_elements
                .into_iter()
                .find(|(entity, content)| **content == UIContent::Content(UIType::SelectionInfo))
                .unwrap()
                .0;
            commands.entity(selection_info_content).clear_children();
            if let Ok(unit_information) = unit_info.get_mut(hit.hit_entity) {
                let infotext = commands
                    .spawn(TextBundle::from_section(
                        format!(
                            "{}\n{}\n{}",
                            unit_information.unit_name,
                            unit_information.civilisation.to_string(),
                            unit_information.unit_type.to_string()
                        ),
                        TextStyle {
                            font: asset_server
                                .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.0, 0.0),
                        },
                    ))
                    .id();
                let thumbnail = commands
                    .spawn(ImageBundle {
                        background_color: NORMAL_BUTTON.into(),
                        style: Style {
                            size: Size {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                            },
                            ..Default::default()
                        },
                        // transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                        image: UiImage {
                            texture: asset_server.load(&unit_information.thumbnail),
                            ..default()
                        },
                        ..Default::default()
                    })
                    .id();
                commands
                    .entity(selection_info_content)
                    .push_children(&[infotext, thumbnail]);
            }
        }
    }
}
// remove all children
fn clear_ui(
    mut commands: Commands,
    ui_elements: Query<(Entity, &UIContent)>,
    deselect_event: EventReader<DeselectEvent>,
) {
    if !deselect_event.is_empty() {
        commands
            .entity(ui_elements
                .into_iter()
                .find(|(entity, content)| **content == UIContent::Content(UIType::SelectionInfo))
                .unwrap()
                .0)
            .clear_children();
    }
}
fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<UIContent>>,
) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        let mut frame_time = time.delta_seconds_f64();
        if let Some(frame_time_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                frame_time = frame_time_smoothed;
            }
        }

        text.sections[0].value = format!("{fps:.1} fps, {frame_time:.3} ms/frame",);
    }
}
