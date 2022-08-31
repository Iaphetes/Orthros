use bevy::{input::mouse::MouseMotion, prelude::*};

use bevy_rapier3d::prelude::*;
use crate::ownable::SelectionCircle;
use crate::ownable::{Selectable, Selected};

pub struct PlayerController;
impl Plugin for PlayerController{
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraController)
            .add_system(mouse_controller);
    }
}



struct CameraController;
impl Plugin for CameraController{
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
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub mouse_key_enable_mouse: MouseButton,
    pub keyboard_key_enable_mouse: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
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
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            mouse_key_enable_mouse: MouseButton::Left,
            keyboard_key_enable_mouse: KeyCode::M,
            walk_speed: 2.0,
            run_speed: 6.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut move_toggled: Local<bool>,
    mut query: Query<(&mut Transform, &mut CameraControllerSettings), With<Camera>>,
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
        // if key_input.pressed(options.key_up) {
        //     axis_input.y += 1.0;
        // }
        // if key_input.pressed(options.key_down) {
        //     axis_input.y -= 1.0;
        // }
        if key_input.just_pressed(options.keyboard_key_enable_mouse) {
            *move_toggled = !*move_toggled;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation +=
            options.velocity.x * dt * right + options.velocity.z * dt * Vec3::Z;

        // Handle mouse input
        // let mut mouse_delta = Vec2::ZERO;
        // if mouse_button_input.pressed(options.mouse_key_enable_mouse) || *move_toggled {
        //     for mouse_event in mouse_events.iter() {
        //         mouse_delta += mouse_event.delta;
        //     }
        // }

        // if mouse_delta != Vec2::ZERO {
        //     // Apply look update
        //     let (pitch, yaw) = (
        //         (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt).clamp(
        //             -0.99 * std::f32::consts::FRAC_PI_2,
        //             0.99 * std::f32::consts::FRAC_PI_2,
        //         ),
        //         options.yaw - mouse_delta.x * options.sensitivity * dt,
        //     );
        //     transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
        //     options.pitch = pitch;
        //     options.yaw = yaw;
        // }
    }
}

fn camera_setup(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        })
        .insert(CameraControllerSettings::default());
}

fn mouse_controller(
    // mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut camera_options: Query<&mut CameraControllerSettings, With<Camera>>,
    windows: Res<Windows>,
    mut selectable: Query<(Entity, &mut Selectable, &Children)>,
    mut selection_circle: Query<&mut Visibility, With<SelectionCircle>>,
    mut selected_entities: Query<&Selected>,
    mut commands: Commands
) {
    if let Ok(options) = camera_options.get_single_mut() {
        if mouse_button_input.pressed(options.mouse_key_enable_mouse){
            for (camera, camera_transform) in cameras.iter() {
                // First, compute a ray from the mouse position.
                let (ray_pos, ray_dir) = ray_from_mouse_position(
                    windows.get_primary().unwrap(),
                    camera,
                    camera_transform,
                );

                // Then cast the ray.
                let hit = rapier_context.cast_ray(
                    ray_pos,
                    ray_dir,
                    f32::MAX,
                    true,
                    QueryFilter::only_dynamic(),
                );
                let mut hit_entity : Option<Entity> = None;
                if let Some((entity, _toi)) = hit {
                    hit_entity = Some(entity.clone());
                    if selected_entities.get_mut(entity).is_err(){
                        if let Ok((_, _select, children)) = selectable.get_mut(entity){
                            for child in children.iter(){
                                if let Ok(mut selection_visibility) = selection_circle.get_mut(*child){
                                    selection_visibility.is_visible = true;
                                    commands.entity(entity).insert(Selected{});
                                }
                            }
                        }
                   }
                }
                for (sel_entity, _, children) in selectable.iter(){

                     let mut deselect : bool = true;
                     match hit_entity{
                         Some(unwrapped) => {
                             if sel_entity == unwrapped{
                                 deselect = false;
                             }
                             
                         }
                         None => {println!{"No assignment"}}

                     }
                     if deselect{
                         for child in children.iter(){
                             if let Ok(mut selection_visibility) = selection_circle.get_mut(*child){
                                     selection_visibility.is_visible = false;
                                     commands.entity(sel_entity).remove::<Selected>();
                                 }
                             }
                         }
                     }
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
