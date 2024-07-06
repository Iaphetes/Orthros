use std::process;

use crate::ownable::{Selectable, Selected};
use crate::player_controller::{ContextMenuAction, LocalPlayer, PlayerInfo};
use crate::player_controller::{DeselectEvent, RayHit, RenderLayerMap};
use crate::resources::{ResourceStockpiles, ResourceType};
use crate::spawner::{
    InstanceSpawnRequest, UnitInformation, UnitSpecification, UnitSpecifications,
};
use bevy::core_pipeline::Skybox;
use bevy::diagnostic::DiagnosticsStore;
use bevy::render::camera::ClearColorConfig;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
const ICON_BACKGROUND: Color = Color::srgb(12.0 / 256.0, 11.0 / 256.0, 13.0 / 256.0);
const NORMAL_BUTTON: Color = Color::WHITE;
const HOVERED_BUTTON: Color = Color::srgb(64.0 / 256.0, 99.0 / 256.0, 64.0 / 256.0);
const PRESSED_BUTTON: Color = Color::srgb(75.0 / 256.0, 110.0 / 256.0, 75.0 / 256.0);
const MAIN_UI_BACKGROUND: Color = Color::srgba(
    0x81 as f32 / 256.0,
    0xC1 as f32 / 256.0,
    0x14 as f32 / 256.0,
    0xF0 as f32 / 256.0,
);
const MAIN_UI_TEXT: Color = Color::srgb(12.0 / 256.0, 11.0 / 256.0, 13.0 / 256.0);
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum UIType {
    MapUI,
    SelectionInfo,
    ContextMenu,
    Resources(ResourceType),
    Diagnostics,
}
#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
enum UIContent {
    Content(UIType),
    Decoration(UIType),
}
#[derive(Event)]
pub struct GameUI;
impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_overlay)
            .add_systems(
                Update,
                (
                    update_fps,
                    populate_lower_ui,
                    clear_ui.before(populate_lower_ui),
                    catch_interaction,
                    button_system,
                    update_resources,
                ),
            )
            .add_event::<RayHit>()
            .add_event::<DeselectEvent>()
            .add_plugins(FrameTimeDiagnosticsPlugin);
    }
}
#[derive(Component)]
pub struct RayBlock;
fn initialise_mini_map(
    commands: &mut Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let size = Extent3d {
        width: 256,
        height: 256,
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

    let skybox_handle: Handle<Image> = asset_server.load("textures/skybox/stacked.png");

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d { ..default() },
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },

            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0))
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        }, // UiCameraConfig { show_ui: false },
        RenderLayers::from_layers(&[
            RenderLayerMap::General as usize,
            RenderLayerMap::Minimap as usize,
        ]),
    ));
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(65.0),
                height: Val::Percent(80.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage::from(image_handle),
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..default()
            });
        })
        .id()
}
fn create_ui_segment(
    commands: &mut Commands,
    style: Style,
    ui_type: UIType,
    background_decoration: Vec<Entity>,
    content: Vec<Entity>,
    foreground_decoration: Vec<Entity>,
) -> Entity {
    println!("Creating {:?} UI", ui_type);
    commands
        .spawn((NodeBundle { style, ..default() }, Interaction::None))
        .with_children(|parent| {
            parent
                .spawn((
                    UIContent::Decoration(ui_type),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),

                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .push_children(&background_decoration);

            parent
                .spawn((
                    UIContent::Content(ui_type),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            top: Val::Percent(0.0),
                            left: Val::Percent(0.0),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .push_children(&content);
            parent
                .spawn((
                    UIContent::Decoration(ui_type),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            top: Val::Percent(0.0),
                            left: Val::Px(0.0),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            // flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .push_children(&foreground_decoration);
        })
        .id()
}
fn game_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: ResMut<Assets<Image>>,
) {
    let map_ui_content: Vec<Entity> = vec![
        // commands.spawn(NodeBundle::default()).id(),
        initialise_mini_map(&mut commands, images, &asset_server),
        // commands.spawn(NodeBundle::default()).id(),
    ];
    let default_column_style: Style = Style {
        width: Val::Percent(10.0),
        height: Val::Percent(120.0),
        ..default()
    };
    let map_ui_decoration: Vec<Entity> = vec![
        commands
            .spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(10.0),
                    height: Val::Percent(120.0),
                    ..default()
                },
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: Style {
                    height: Val::Percent(80.0),
                    width: Val::Percent(65.0),
                    ..default()
                },
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/map_decoration.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: default_column_style.clone(),
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
    ];
    let context_menu_decoration: Vec<Entity> = vec![
        commands
            .spawn(ImageBundle {
                style: default_column_style.clone(),
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(1000.0),
                    height: Val::Percent(80.0),
                    ..default()
                },
                background_color: MAIN_UI_BACKGROUND.into(),
                ..default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: default_column_style.clone(),
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
    ];
    let selection_info_decoration: Vec<Entity> = vec![
        commands
            .spawn(ImageBundle {
                style: default_column_style.clone(),
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(1000.0),
                    height: Val::Percent(80.0),
                    ..default()
                },
                background_color: MAIN_UI_BACKGROUND.into(),
                ..default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: default_column_style.clone(),
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
    ];
    let diagnostics_decoration: Vec<Entity> = vec![commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(800.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: MAIN_UI_BACKGROUND.into(),
            ..default()
        })
        .id()];
    let diagnostics_content: Vec<Entity> = vec![commands
        .spawn((
            UIContent::Content(UIType::Diagnostics),
            TextBundle::from_section(
                "FPS".to_string(),
                TextStyle {
                    font: asset_server
                        .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                    font_size: 20.0,
                    color: MAIN_UI_TEXT,
                },
            ),
        ))
        .id()];
    let resources_decoration: Vec<Entity> = vec![commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(800.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: MAIN_UI_BACKGROUND.into(),
            ..default()
        })
        .id()];

    let resources_content: Vec<Entity> = vec![
        commands
            .spawn((
                UIContent::Content(UIType::Resources(ResourceType::Plotanium)),
                ImageBundle {
                    style: Style {
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        ..Default::default()
                    },
                    image: UiImage {
                        texture: asset_server.load("textures/ui/resources/resource_a.png"),
                        ..default()
                    },
                    ..Default::default()
                },
            ))
            .id(),
        commands
            .spawn((
                UIContent::Content(UIType::Resources(ResourceType::Plotanium)),
                TextBundle::from_section(
                    "0".to_string(),
                    TextStyle {
                        font: asset_server
                            .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                        font_size: 20.0,
                        color: MAIN_UI_TEXT,
                    },
                ),
            ))
            .id(),
    ];
    let top_ui_elements: Vec<Entity> = vec![
        create_ui_segment(
            &mut commands,
            Style {
                width: Val::Percent(10.0),
                height: Val::Percent(100.0),
                top: Val::Percent(0.0),
                left: Val::Px(0.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            UIType::Diagnostics,
            diagnostics_decoration,
            diagnostics_content,
            Vec::new(),
        ),
        create_ui_segment(
            &mut commands,
            Style {
                width: Val::Percent(10.0),
                height: Val::Percent(100.0),
                top: Val::Percent(0.0),
                left: Val::Px(0.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            UIType::Diagnostics,
            resources_decoration,
            resources_content,
            Vec::new(),
        ),
    ];
    let lower_ui_elements: Vec<Entity> = vec![
        create_ui_segment(
            &mut commands,
            Style {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                top: Val::Percent(0.0),
                left: Val::Px(0.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            UIType::ContextMenu,
            context_menu_decoration,
            // map_ui_content.clone(),
            Vec::new(),
            Vec::new(),
        ),
        create_ui_segment(
            &mut commands,
            Style {
                width: Val::Percent(30.0),
                height: Val::Percent(100.0),
                top: Val::Percent(0.0),
                left: Val::Px(0.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            UIType::SelectionInfo,
            selection_info_decoration,
            // map_ui_content.clone(),
            Vec::new(),
            Vec::new(),
        ),
        create_ui_segment(
            &mut commands,
            Style {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                top: Val::Percent(0.0),
                left: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            UIType::MapUI,
            Vec::new(),
            map_ui_content,
            map_ui_decoration,
        ),
    ];
    //Full UI
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top UI
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(5.0),

                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    ..default()
                })
                .push_children(&top_ui_elements);
            // Lower UI
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(15.0),
                        top: Val::Percent(80.0),
                        left: Val::Px(0.0),
                        justify_content: JustifyContent::SpaceBetween,
                        flex_wrap: FlexWrap::Wrap,
                        align_content: AlignContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .push_children(&lower_ui_elements);
        });
}
fn update_selection_info(
    commands: &mut Commands,
    unit_information: &UnitInformation,
    asset_server: &Res<AssetServer>,
    selection_info_content: Entity,
) {
    let infotext = commands
        .spawn(TextBundle::from_section(
            format!(
                "{}\n{}\n{}",
                unit_information.unit_name,
                unit_information.civilisation,
                unit_information.unit_type
            ),
            TextStyle {
                font: asset_server.load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                font_size: 20.0,
                color: MAIN_UI_TEXT,
            },
        ))
        .id();
    let thumbnail = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(60.0),
                height: Val::Px(60.0),
                ..default()
            },
            background_color: ICON_BACKGROUND.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                image: UiImage {
                    texture: asset_server.load(&unit_information.thumbnail),
                    ..default()
                },
                ..Default::default()
            });
        })
        .id();
    let container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(75.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .push_children(&[infotext, thumbnail])
        .id();
    commands.entity(selection_info_content).add_child(container);
}
fn catch_interaction(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, Changed<Interaction>>,
    rayblock: Query<Entity, With<RayBlock>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed | Interaction::Hovered => {
                commands.spawn(RayBlock);
                println!("Catching Rays");
            }
            Interaction::None => {
                for block in rayblock.iter() {
                    commands.entity(block).despawn_recursive();
                }
            }
        }
    }
}
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &ContextMenuAction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    player_info: Query<&PlayerInfo, With<LocalPlayer>>,
    selected_entities: Query<&Transform, With<Selected>>,
    mut spawn_events: EventWriter<InstanceSpawnRequest>,
) {
    if let Ok(player_info) = player_info.get_single() {
        for transform in selected_entities.iter() {
            for (interaction, action, mut background_color, mut border_color) in
                &mut interaction_query
            {
                match *interaction {
                    Interaction::Pressed => {
                        match action {
                            ContextMenuAction::Build(unit_type) => {
                                spawn_events.send(InstanceSpawnRequest {
                                    location: Vec3 {
                                        x: transform.translation.x + 2.0,
                                        y: 2.0,
                                        z: transform.translation.z + 1.0,
                                    },
                                    unit_type: unit_type.clone(),
                                    civilisation: player_info.civilisation,
                                });
                            }
                        };
                        *background_color = PRESSED_BUTTON.into();
                        border_color.0 = Color::BLACK;
                    }
                    Interaction::Hovered => {
                        border_color.0 = Color::BLACK;
                        *background_color = HOVERED_BUTTON.into();
                    }
                    Interaction::None => {
                        border_color.0 = Color::BLACK;
                        *background_color = NORMAL_BUTTON.into();
                    }
                }
            }
        }
    }
}

fn update_context_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    context_menu_content: Entity,
    context_menu_actions: &Vec<ContextMenuAction>,
    unit_specifications: &Res<UnitSpecifications>,
    player_info: &PlayerInfo,
) {
    let mut buttons: Vec<Entity> = Vec::new();
    for action in context_menu_actions {
        match action {
            ContextMenuAction::Build(unit_type) => {
                let unit_information: &UnitSpecification = &unit_specifications.unit_specifications
                    [&(player_info.civilisation, unit_type.clone())];
                buttons.push(
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(70.0),
                                height: Val::Px(70.0),
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            // background_color: ICON_BACKGROUND.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(65.0),
                                        height: Val::Px(65.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::percent(5.0, 5.0, 5.0, 5.0),
                                        ..default()
                                    },
                                    image: UiImage {
                                        texture: asset_server.load(&unit_information.icon_path),
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    border_color: Color::BLACK.into(),
                                    ..default()
                                },
                                action.clone(),
                            ));
                        })
                        .id(),
                );
                let container = commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            height: Val::Percent(80.0),
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    })
                    .push_children(&buttons)
                    .id();
                commands.entity(context_menu_content).add_child(container);
            }
        }
    }
}
fn populate_lower_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ray_hit_event: EventReader<RayHit>,
    mut unit_info: Query<&UnitInformation, With<Selectable>>,
    ui_elements: Query<(Entity, &UIContent)>,
    player_info: Query<&PlayerInfo, With<LocalPlayer>>,
    unit_specifications: Res<UnitSpecifications>,
) {
    if let Ok(player_info) = player_info.get_single() {
        for hit in ray_hit_event.read() {
            if hit.mouse_key_enable_mouse {
                let (selection_info_content, _): (Entity, _) = ui_elements
                    .into_iter()
                    .find(|(_, content)| **content == UIContent::Content(UIType::SelectionInfo))
                    .unwrap();
                commands
                    .entity(selection_info_content)
                    .despawn_descendants();
                let (context_menu_content, _): (Entity, _) = ui_elements
                    .into_iter()
                    .find(|(_, content)| **content == UIContent::Content(UIType::ContextMenu))
                    .unwrap();
                commands.entity(context_menu_content).despawn_descendants();
                if let Ok(unit_information) = unit_info.get_mut(hit.hit_entity) {
                    update_selection_info(
                        &mut commands,
                        unit_information,
                        &asset_server,
                        selection_info_content,
                    );
                    if let Some(contex_menu_actions) = player_info
                        .context_menu_actions
                        .get(&unit_information.unit_type)
                    {
                        update_context_menu(
                            &mut commands,
                            &asset_server,
                            context_menu_content,
                            contex_menu_actions,
                            &unit_specifications,
                            player_info,
                        );
                    }
                } else {
                    commands.entity(selection_info_content).push_children(&[]);
                }
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
        let (selection_info_content, _): (Entity, _) = ui_elements
            .into_iter()
            .find(|(_, content)| **content == UIContent::Content(UIType::SelectionInfo))
            .unwrap();
        commands
            .entity(selection_info_content)
            .despawn_descendants();
        let (contect_menu_content, _): (Entity, _) = ui_elements
            .into_iter()
            .find(|(_, content)| **content == UIContent::Content(UIType::ContextMenu))
            .unwrap();
        commands.entity(contect_menu_content).despawn_descendants();
    }
}
fn update_fps(diagnostics: Res<DiagnosticsStore>, mut query: Query<(&mut Text, &UIContent)>) {
    for (mut text, ui_content) in &mut query {
        if let UIContent::Content(UIType::Diagnostics) = ui_content {
            let mut fps = 0.0;
            if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                    fps = fps_smoothed;
                }
            }

            text.sections[0].value = format!("{fps:.1} fps",);
        }
    }
}

fn update_resources(
    localplayer: Query<&ResourceStockpiles, With<LocalPlayer>>,
    mut ui_elements: Query<(&mut Text, &UIContent)>,
) {
    if let Ok(resource_stockpiles) = localplayer.get_single() {
        for (mut text, ui_content) in &mut ui_elements {
            if let UIContent::Content(UIType::Resources(resource_type)) = ui_content {
                if let Some(resource_amount) = resource_stockpiles.get(&resource_type) {
                    text.sections[0].value = format!("{}", *resource_amount);
                }
            }
        }
    } else {
        error!("No local player found");
        process::exit(1);
    }
}
