#![feature(let_chains)]
use crate::environment::MovementGrid;
use crate::movable::{Movable, MoveCommand};
use crate::ownable::SelectionCircle;
use crate::ownable::{Selectable, Selected};
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::math::Quat;
use bevy::window::PrimaryWindow;
use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
};
use bevy_rapier3d::prelude::*;
pub struct PlayerController;
impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraController)
            .add_event::<RayHit>()
            .add_startup_system(game_overlay)
            .add_system(process_mouse)
            .add_system(mouse_controller.after(process_mouse));
    }
}

struct CameraController;
impl Plugin for CameraController {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup)
            .add_system(camera_controller);
    }
}

#[derive(Component)]
pub struct CameraControllerSettings {
    pub enabled: bool,
    pub initialized: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub rotate_key: KeyCode,
    pub rotation_speed: f32,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    pub mouse_unit_move_button: MouseButton,
    pub keyboard_key_enable_mouse: KeyCode,
    pub pan_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub zoom_min: f32,
    pub zoom_max: f32,
    pub zoom_speed: f32,
}

impl Default for CameraControllerSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            initialized: false,
            sensitivity: 0.5,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            rotate_key: KeyCode::LControl,
            rotation_speed: 0.005,
            key_run: KeyCode::LShift,
            mouse_key_enable_mouse: MouseButton::Left,
            mouse_unit_move_button: MouseButton::Right,
            keyboard_key_enable_mouse: KeyCode::M,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            pan_speed: 4.0,
            zoom_speed: 50.0,
            zoom_min: 5.0,
            zoom_max: 100.0,
        }
    }
}

pub fn camera_controller(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut move_toggled: Local<bool>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut Transform, &mut CameraControllerSettings), With<Camera>>,
    rapier_context: Res<RapierContext>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            options.yaw = yaw;
            options.pitch = pitch;
            options.initialized = true;
        }
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        let mut yaw: f32 = 0.0;
        let mut pitch: f32 = 0.0;
        // let mut roll: f32 = 0.0;

        if key_input.pressed(options.rotate_key) {
            if key_input.pressed(options.key_forward) {
                pitch -= 1.0
            }
            if key_input.pressed(options.key_back) {
                pitch += 1.0
            }
            if key_input.pressed(options.key_right) {
                yaw += 1.0;
            }
            if key_input.pressed(options.key_left) {
                yaw -= 1.0;
            }
        } else {
            if key_input.pressed(options.key_forward) {
                axis_input.z += 1.0;
            }
            if key_input.pressed(options.key_back) {
                axis_input.z -= 1.0;
            }
            if key_input.pressed(options.key_right) {
                axis_input.x += 1.0;
            }
            if key_input.pressed(options.key_left) {
                axis_input.x -= 1.0;
            }
        }

        if key_input.just_pressed(options.keyboard_key_enable_mouse) {
            *move_toggled = !*move_toggled;
        }
        for evt in mouse_wheel.iter() {
            match evt.unit {
                MouseScrollUnit::Line => {
                    if (transform.translation.y > options.zoom_min || evt.y < 0.0)
                        && (transform.translation.y < options.zoom_max || evt.y > 0.0)
                    {
                        axis_input.y = -evt.y;
                    }
                }
                MouseScrollUnit::Pixel => {}
            }
        }
        // Apply movement update
        if axis_input != Vec3::ZERO {
            options.velocity = axis_input.normalize()
                * Vec3 {
                    x: options.pan_speed,
                    y: options.zoom_speed,
                    z: options.pan_speed,
                };
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let right = transform.right();

        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * Vec3::Z;
        if key_input.pressed(options.rotate_key) {
            for (camera, camera_transform) in cameras.iter() {
                // First, compute a ray from the mouse position.
                let (ray_pos, ray_dir) = ray_from_camera_center(camera, camera_transform);
                let intersection: Option<(Entity, RayIntersection)> = rapier_context
                    .cast_ray_and_get_normal(
                        ray_pos,
                        ray_dir,
                        f32::MAX,
                        true,
                        QueryFilter::exclude_solids(QueryFilter::new()),
                    );
                match intersection {
                    Some((_, rayintersection)) => {
                        let rot: Quat = Quat::from_rotation_x(pitch * options.rotation_speed)
                            * Quat::from_rotation_y(yaw * options.rotation_speed);
                        transform.rotate_around(rayintersection.point, rot);
                    }
                    None => {
                        println!("Not rotating");
                    }
                }
            }
        }
    }
}

fn camera_setup(mut commands: Commands) {
    // camera
    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
                // tonemapping: Tonemapping::TonyMcMapface,
                ..default()
            },
            BloomSettings::default(),
        ))
        .insert(CameraControllerSettings::default());
}

fn mouse_controller(
    // mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut camera_options: Query<&mut CameraControllerSettings, With<Camera>>,
    mut selectable: Query<(Entity, &mut Selectable, &Children)>,
    mut selection_circle: Query<&mut Visibility, With<SelectionCircle>>,
    mut selected_entities: Query<(Entity, &Selected)>,
    mut movables: Query<Entity, (With<Selected>, With<Movable>)>,
    mut commands: Commands,
    gridmap: Res<MovementGrid>,
    mut ray_hit_event: EventReader<RayHit>,
) {
    for hit in ray_hit_event.iter() {
        if hit.mouse_key_enable_mouse && selected_entities.get_mut(hit.hit_entity).is_err() {
            if let Ok((_, _select, children)) = selectable.get_mut(hit.hit_entity) {
                for child in children.iter() {
                    if let Ok(mut selection_visibility) = selection_circle.get_mut(*child) {
                        *selection_visibility = Visibility::Visible;
                        commands.entity(hit.hit_entity).insert(Selected {});
                    }
                }
            }
            for (sel_entity, _, children) in selectable.iter() {
                let mut deselect: bool = true;

                if sel_entity == hit.hit_entity {
                    deselect = false;
                }
                if deselect {
                    for child in children.iter() {
                        if let Ok(mut selection_visibility) = selection_circle.get_mut(*child) {
                            *selection_visibility = Visibility::Hidden;
                            commands.entity(sel_entity).remove::<Selected>();
                        }
                    }
                }
            }
        }

        // }

        if hit.mouse_unit_move_button {
            println!("Move");
            let target: Vec2 = Vec2 {
                x: hit.ray_intersection.point.x,
                y: hit.ray_intersection.point.z,
            };

            for entity in movables.iter_mut() {
                commands.entity(entity).remove::<MoveCommand>();
                commands.entity(entity).insert(MoveCommand {
                    target: target.clone(),
                });
            }
        }
    }
}

// Credit to @doomy on discord.
fn ray_from_mouse_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> (Vec3, Vec3) {
    let mouse_position = window.cursor_position().unwrap_or(Vec2::new(0.0, 0.0));
    let x = 2.0 * (mouse_position.x / window.width() as f32) - 1.0;
    let y = 2.0 * (mouse_position.y / window.height() as f32) - 1.0;

    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}

fn ray_from_camera_center(camera: &Camera, camera_transform: &GlobalTransform) -> (Vec3, Vec3) {
    let x = 0.0;
    let y = 0.0;
    println!("x{}, y{}", x, y);
    let camera_inverse_matrix =
        camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let near = camera_inverse_matrix * Vec3::new(x, y, -1.0).extend(1.0);
    let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

    let near = near.truncate() / near.w;
    let far = far.truncate() / far.w;
    let dir: Vec3 = far - near;
    (near, dir)
}

struct RayHit {
    hit_entity: Entity,
    mouse_key_enable_mouse: bool,
    mouse_unit_move_button: bool,
    ray_intersection: RayIntersection,
}
fn handle_select(
    primary: &Window,
    rapier_context: &Res<RapierContext>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    ray_hit_event: &mut EventWriter<RayHit>,
    mouse_unit_move_button: bool,
    mouse_key_enable_mouse: bool,
) {
    let (ray_pos, ray_dir) = ray_from_mouse_position(primary, camera, camera_transform);
    // println!("{:?}", mouse_unit_move_button);
    // Then cast the ray.
    let hit = rapier_context.cast_ray_and_get_normal(
        ray_pos,
        ray_dir,
        f32::MAX,
        true,
        QueryFilter::only_dynamic(),
    );
    //Make also sensor cast...
    // let mut hit_entity: Option<Entity> = None;
    if let Some((hit_entity, ray_intersection)) = hit {
        println!("Send event");
        ray_hit_event.send(RayHit {
            hit_entity,
            mouse_unit_move_button,
            mouse_key_enable_mouse,
            ray_intersection,
        })
    }
}
fn handle_unit_move_cmd(
    primary: &Window,
    rapier_context: &Res<RapierContext>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    ray_hit_event: &mut EventWriter<RayHit>,
    mouse_unit_move_button: bool,
    mouse_key_enable_mouse: bool,
) {
    let (ray_pos, ray_dir) = ray_from_mouse_position(primary, camera, camera_transform);

    let hit = rapier_context.cast_ray_and_get_normal(
        ray_pos,
        ray_dir,
        f32::MAX,
        true,
        QueryFilter::exclude_solids(QueryFilter::new()),
    ); //Make also sensor cast...
    let mut hit_entity: Option<Entity> = None;
    if let Some((hit_entity, ray_intersection)) = hit {
        println!("Send event");
        ray_hit_event.send(RayHit {
            hit_entity,
            mouse_unit_move_button,
            mouse_key_enable_mouse,
            ray_intersection,
        })
    }
}
fn process_mouse(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &Children)>,
    mut text_query: Query<&mut Text>,
    mut ray_hit_event: EventWriter<RayHit>,
    mouse_button_input: Res<Input<MouseButton>>,
    camera_options: Query<(&CameraControllerSettings, &Camera, &GlobalTransform)>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };

    let mut mouse_over_ui: bool = false;
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                mouse_over_ui = true;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                mouse_over_ui = true;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
    if let Ok((options, camera, camera_transform)) = camera_options.get_single() {
        let mouse_key_enable_mouse =
            mouse_button_input.just_pressed(options.mouse_key_enable_mouse);
        let mouse_unit_move_button =
            mouse_button_input.just_pressed(options.mouse_unit_move_button);
        if mouse_over_ui {
            return;
        }
        if mouse_key_enable_mouse {
            handle_select(
                &primary,
                &rapier_context,
                camera,
                camera_transform,
                &mut ray_hit_event,
                mouse_unit_move_button,
                mouse_key_enable_mouse,
            )
        }
        if mouse_unit_move_button {
            handle_unit_move_cmd(
                &primary,
                &rapier_context,
                camera,
                camera_transform,
                &mut ray_hit_event,
                mouse_unit_move_button,
                mouse_key_enable_mouse,
            )
        }
    }
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
