use crate::ownable::Selectable;
use crate::player_controller::{DeselectEvent, RayHit};
use crate::spawner::UnitInformation;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct BuildUI;
#[derive(Component)]
struct UnitInfoUI;
#[derive(Component)]
struct FPSCounter;

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

fn game_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        TextBundle::from_section(
                            format!("FPS - ms/Frame"),
                            TextStyle {
                                font: asset_server
                                    .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                        FPSCounter,
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
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        ..default()
                    },
                    background_color: Color::rgb(1.0, 1.0, 1.0).into(),

                    ..default()
                })
                .with_children(|parent| {
                    // Left part (Build menu)
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(33.0), Val::Percent(100.0)),
                                position: UiRect {
                                    top: Val::Percent(0.0),
                                    left: Val::Px(0.0),
                                    ..default()
                                },
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            background_color: Color::rgb(1.0, 0.0, 0.0).into(),

                            ..default()
                        },
                        BuildUI,
                    ));
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(33.0), Val::Percent(100.0)),
                                position: UiRect {
                                    top: Val::Percent(0.0),
                                    left: Val::Px(0.0),
                                    ..default()
                                },
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            background_color: Color::rgb(0.0, 1.0, 0.0).into(),

                            ..default()
                        },
                        UnitInfoUI,
                    ));
                    // .with_children(|parent| {
                    //     parent
                    //         .spawn(ButtonBundle {
                    //             style: Style {
                    //                 size: Size::new(Val::Percent(5.0), Val::Percent(20.0)),
                    //                 position: UiRect {
                    //                     top: Val::Percent(0.0),
                    //                     right: Val::Percent(0.0),
                    //                     ..default()
                    //                 },
                    //                 margin: UiRect {
                    //                     top: Val::Px(5.0),
                    //                     right: Val::Px(5.0),
                    //                     left: Val::Px(5.0),
                    //                     bottom: Val::Px(5.0)
                    //                 },
                    //                 // horizontally center child text
                    //                 justify_content: JustifyContent::Center,
                    //                 // vertically center child text
                    //                 align_items: AlignItems::Center,
                    //                 ..default()
                    //             },
                    //             background_color: NORMAL_BUTTON.into(),
                    //             // image: UiImage {
                    //             //     texture: asset_server.load("textures/selection_texture.png"),
                    //             //     ..default()
                    //             // },
                    //             ..default()
                    //         })
                    //         .with_children(|parent| {
                    //             // parent.spawn(TextBundle::from_section(
                    //             //     "Button",
                    //             //     TextStyle {
                    //             //         font: asset_server
                    //             //             .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                    //             //         font_size: 40.0,
                    //             //         color: Color::rgb(0.9, 0.9, 0.9),
                    //             //     },
                    //             // ));
                    //             parent.spawn(ImageBundle {
                    //                 style: Style {
                    //                     size: Size::new(Val::Px(20.0), Val::Px(20.0)),
                    //                     align_self: AlignSelf::Center,
                    //                     ..Default::default()
                    //                 },
                    //                 background_color: NORMAL_BUTTON.into(),
                    //                 transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                    //                 image: UiImage {
                    //                     texture: asset_server.load("textures/selection_texture.png"),
                    //                     ..default()
                    //                 },
                    //                 ..Default::default()
                    //             });
                    //         });
                    // parent.spawn(bundle)
                    // });
                });
        });
    // Main lower window
}

fn populate_lower_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ray_hit_event: EventReader<RayHit>,
    mut unit_info: Query<&UnitInformation, With<Selectable>>,
    unit_info_ui: Query<Entity, With<UnitInfoUI>>,
) {
    for hit in ray_hit_event.iter() {
        if hit.mouse_key_enable_mouse {
            commands
                .entity(unit_info_ui.get_single().unwrap())
                .clear_children();
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

                commands
                    .entity(unit_info_ui.get_single().unwrap())
                    .push_children(&[infotext]);
            }
        }
    }
}
fn clear_ui(
    mut commands: Commands,
    unit_info_ui: Query<Entity, With<UnitInfoUI>>,
    deselect_event: EventReader<DeselectEvent>,
) {
    if !deselect_event.is_empty() {
        commands
            .entity(unit_info_ui.get_single().unwrap())
            .clear_children();
    }
}
fn change_text_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FPSCounter>>,
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
