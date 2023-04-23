use crate::ownable::Selectable;
use crate::player_controller::{DeselectEvent, RayHit, RenderLayerMap};
use crate::spawner::{UnitInformation, UnitSpecification, UnitSpecifications, UnitType};
use crate::{ContextMenuAction, PlayerInfo};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::utils::HashMap;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
const NORMAL_BUTTON: Color = Color::rgb(12.0 / 256.0, 11.0 / 256.0, 13.0 / 256.0);
const MAIN_UI_BACKGROUND: Color = Color::rgba(
    0x81 as f32 / 256.0,
    0xC1 as f32 / 256.0,
    0x14 as f32 / 256.0,
    0xF0 as f32 / 256.0,
);
const MAIN_UI_TEXT: Color = Color::rgb(12.0 / 256.0, 11.0 / 256.0, 13.0 / 256.0);
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
                    width: Val::Percent(65.0),
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
    background_decoration: Vec<Entity>,

    content: Vec<Entity>,
    foreground_decoration: Vec<Entity>,
) -> Entity {
    commands
        .spawn((NodeBundle { style, ..default() },))
        .with_children(|parent| {
            parent
                .spawn((
                    UIContent::Decoration(ui_type),
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            // position_type: PositionType::Absolute,
                            align_items: AlignItems::FlexEnd,
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Row,
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
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            position: UiRect {
                                top: Val::Percent(0.0),
                                left: Val::Px(0.0),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::FlexEnd,
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Row,
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
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            position: UiRect {
                                top: Val::Percent(0.0),
                                left: Val::Px(0.0),
                                ..default()
                            },
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::FlexEnd,
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Row,
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
        commands.spawn(NodeBundle::default()).id(),
        initialise_mini_map(&mut commands, images),
        commands.spawn(NodeBundle::default()).id(),
    ];
    let map_ui_decoration: Vec<Entity> = vec![
        commands
            .spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(10.0), Val::Percent(120.0)),
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
                    size: Size {
                        height: Val::Percent(100.0),
                        width: Val::Percent(65.0),
                    },
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
                style: Style {
                    size: Size::new(Val::Percent(10.0), Val::Percent(120.0)),
                    ..default()
                },
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
                style: Style {
                    size: Size::new(Val::Percent(10.0), Val::Percent(120.0)),
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
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(800.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: MAIN_UI_BACKGROUND.into(),
                ..default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(10.0), Val::Percent(120.0)),
                    ..default()
                },
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
                style: Style {
                    size: Size::new(Val::Percent(10.0), Val::Percent(120.0)),
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
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(800.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: MAIN_UI_BACKGROUND.into(),
                ..default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(10.0), Val::Percent(120.0)),
                    ..default()
                },
                image: UiImage {
                    texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
                    ..default()
                },
                ..default()
            })
            .id(),
    ];
    let diagnostics_decoration: Vec<Entity> = vec![
        // commands
        //     .spawn(ImageBundle {
        //         style: Style {
        //             size: Size::new(Val::Percent(10.0), Val::Percent(120.0)),
        //             ..default()
        //         },
        //         image: UiImage {
        //             texture: asset_server.load("textures/ui/greek/context_menu_decoration_b.png"),
        //             ..default()
        //         },
        //         ..default()
        //     })
        //     .id(),
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(800.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: MAIN_UI_BACKGROUND.into(),
                ..default()
            })
            .id(),
    ];
    let diagnostics_content: Vec<Entity> = vec![commands
        .spawn((
            UIContent::Content(UIType::Diagnostics),
            TextBundle::from_section(
                format!("FPS"),
                TextStyle {
                    font: asset_server
                        .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                    font_size: 20.0,
                    color: MAIN_UI_TEXT,
                },
            ),
        ))
        .id()];
    let top_ui_elements: Vec<Entity> = vec![create_ui_segment(
        &mut commands,
        Style {
            size: Size::new(Val::Percent(10.0), Val::Percent(100.0)),
            position: UiRect {
                top: Val::Percent(0.0),
                left: Val::Px(0.0),
                ..default()
            },
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        },
        UIType::Diagnostics,
        diagnostics_decoration,
        diagnostics_content,
        Vec::new(),
    )];
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
            context_menu_decoration,
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
            UIType::SelectionInfo,
            selection_info_decoration,
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
            Vec::new(),
            map_ui_content,
            map_ui_decoration,
        ),
    ];
    //Full UI
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),

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
                        size: Size::new(Val::Percent(50.0), Val::Percent(5.0)),

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
                        size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                        position: UiRect {
                            top: Val::Percent(80.0),
                            left: Val::Px(0.0),
                            ..default()
                        },
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
                unit_information.civilisation.to_string(),
                unit_information.unit_type.to_string()
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
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                },
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
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
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .push_children(&[infotext, thumbnail])
        .id();
    commands.entity(selection_info_content).add_child(container);
}
fn update_context_menu(
    commands: &mut Commands,
    unit_information: &UnitInformation,
    asset_server: &Res<AssetServer>,
    context_menu_content: Entity,
    context_menu_actions: &Vec<ContextMenuAction>,
    unit_specifications: &Res<UnitSpecifications>,
    player_info: &Res<PlayerInfo>,
) {
    let mut buttons: Vec<Entity> = Vec::new();
    for action in context_menu_actions {
        match action {
            ContextMenuAction::BUILD(unit_type) => {
                let unit_information: &UnitSpecification = &unit_specifications.unit_specifications
                    [&(player_info.civilisation, *unit_type)];
                buttons.push(
                    commands
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(65.0), Val::Px(65.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            image: UiImage {
                                texture: asset_server.load(&unit_information.icon_path),
                                ..default()
                            },
                            background_color: Color::BLACK.into(),
                            ..default()
                        })
                        .id(),
                );
            }
        }
    }
    let container = commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .push_children(&buttons)
        .id();
    commands.entity(context_menu_content).add_child(container);
}
fn populate_lower_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ray_hit_event: EventReader<RayHit>,
    mut unit_info: Query<&UnitInformation, With<Selectable>>,
    ui_elements: Query<(Entity, &UIContent, &Children)>,
    ui_children: Query<Entity, With<Style>>,
    player_info: Res<PlayerInfo>,
    unit_specifications: Res<UnitSpecifications>,
) {
    for hit in ray_hit_event.iter() {
        if hit.mouse_key_enable_mouse {
            let (selection_info_content, _, children): (Entity, _, &Children) = ui_elements
                .into_iter()
                .find(|(_, content, _)| **content == UIContent::Content(UIType::SelectionInfo))
                .unwrap();
            for &child in children.iter() {
                println!("{:#?}", child);
                match ui_children.get(child) {
                    Ok(child) => {
                        commands.entity(child).despawn_recursive();
                    }
                    Err(_) => {}
                }
            }
            let (context_menu_content, _, children): (Entity, _, &Children) = ui_elements
                .into_iter()
                .find(|(_, content, _)| **content == UIContent::Content(UIType::ContextMenu))
                .unwrap();
            for &child in children.iter() {
                println!("{:#?}", child);
                match ui_children.get(child) {
                    Ok(child) => {
                        commands.entity(child).despawn_recursive();
                    }
                    Err(_) => {}
                }
            }
            if let Ok(unit_information) = unit_info.get_mut(hit.hit_entity) {
                update_selection_info(
                    &mut commands,
                    &unit_information,
                    &asset_server,
                    selection_info_content,
                );
                match player_info
                    .context_menu_actions
                    .get(&unit_information.unit_type)
                {
                    Some(contex_menu_actions) => update_context_menu(
                        &mut commands,
                        &unit_information,
                        &asset_server,
                        context_menu_content,
                        &contex_menu_actions,
                        &unit_specifications,
                        &player_info,
                    ),
                    None => {}
                }
            } else {
                commands.entity(selection_info_content).push_children(&[]);
            }
        }
    }
}
// remove all children
fn clear_ui(
    mut commands: Commands,
    ui_elements: Query<(Entity, &UIContent, &Children)>,
    ui_children: Query<Entity, With<Style>>,
    deselect_event: EventReader<DeselectEvent>,
) {
    if !deselect_event.is_empty() {
        let (_, _, children): (_, _, &Children) = ui_elements
            .into_iter()
            .find(|(_, content, _)| **content == UIContent::Content(UIType::SelectionInfo))
            .unwrap();
        for &child in children.iter() {
            match ui_children.get(child) {
                Ok(child) => {
                    commands.entity(child).despawn_recursive();
                }
                Err(_) => {}
            }
        }
        let (context_menu_content, _, children): (Entity, _, &Children) = ui_elements
            .into_iter()
            .find(|(_, content, _)| **content == UIContent::Content(UIType::ContextMenu))
            .unwrap();
        for &child in children.iter() {
            println!("{:#?}", child);
            match ui_children.get(child) {
                Ok(child) => {
                    commands.entity(child).despawn_recursive();
                }
                Err(_) => {}
            }
        }
    }
}
fn change_text_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<UIContent>>) {
    for mut text in &mut query {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                fps = fps_smoothed;
            }
        }

        text.sections[0].value = format!("{fps:.1} fps",);
    }
}
