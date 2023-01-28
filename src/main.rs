//! Load a cubemap texture onto a cube like a skybox and cycle through different compressed texture formats
mod environment;
mod movable;
mod ownable;
mod player_controller;
mod skybox;
mod spawner;

use bevy::prelude::*;

//use bevy::render::render_resource::Texture;
use crate::environment::Environment;
use crate::movable::UnitMovement;
// use crate::movable::{move_units, MoveTarget};
use crate::ownable::{Selectable, SelectionCircle};
use crate::player_controller::PlayerController;
use crate::spawner::Instance_Spawner;
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(PlayerController)
        .add_plugin(Environment)
        .add_plugin(UnitMovement)
        .add_plugin(Instance_Spawner)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(game_overlay)
        .add_system(button_system)
        // .add_system(update_emissiveness)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    loaded_units: Query<(Entity, &Handle<StandardMaterial>, &Name)>,
) {
    let texture_handle = asset_server.load("textures/selection_texture.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });
    let scene_handle: Handle<Scene> =
        asset_server.load("../assets/3d_models/units/fighter_01.glb#Scene0");
    // let mut scene : Scene = scene_handle.get_field_mut("Scene").unwrap();
    let parent_id = commands
        .spawn(
            //
            (
                // Transform::from_xyz(0.0, f32::MAX, 0.0).with_scale(Vec3::splat(0.2)),
                SceneBundle {
                    transform: Transform::from_xyz(0.0, 2.0, 0.0).with_scale(Vec3::splat(0.2)),
                    scene: asset_server.load("../assets/3d_models/units/fighter_01.gltf#Scene0"),
                    //scene: asset_server.load("../assets/3d_models/units/untitled.glb#Scene0"),
                    ..default()
                },
                Selectable {},
                RigidBody::Dynamic,
                Collider::capsule_z(1.0, 1.5),
                GravityScale(0.0),
            ),
        )
        .id();
    let child_id = commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(shape::Plane { size: 5. }.into()),
            material: material_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(SelectionCircle {})
        .id();
    commands.entity(parent_id).push_children(&[child_id]);
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn game_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::bottom(Val::Percent(1.5)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Button",
                TextStyle {
                    font: asset_server
                        .load("fonts/android-insomnia-font/AndroidInsomniaRegular.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
