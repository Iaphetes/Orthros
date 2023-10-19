use crate::a_star::{a_star, calculate_a_star};
use crate::environment::MovementGrid;
use bevy::ecs::component::Component;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::transform::components::Transform;
use std::f32::consts::PI;
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
pub struct UnitMovement;

impl Plugin for UnitMovement {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (a_star, calculate_a_star))
            .add_systems(Update, move_units)
            .insert_resource(MovementTimer(Timer::new(
                Duration::from_millis(1500),
                TimerMode::Repeating,
            )));
    }
}
pub const DISTANCE_FACTOR: f32 = 1.0;
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct NodeCoords {
    pub xy: UVec2,
    pub h: Option<Heading>,
}
#[derive(Debug, Clone, Copy)]
pub struct PathNode {
    pub xy: Vec2,
    pub h: Heading,
}
#[derive(Eq, PartialEq, Hash, Clone, Copy, EnumIter, Debug, Default)]
pub enum Heading {
    #[default]
    N,
    // NNE,
    NE,
    // NEE,
    E,
    // SEE,
    SE,
    // SSE,
    S,
    // SSW,
    SW,
    // SWW,
    W,
    // NWW,
    NW,
}
#[derive(Component)]
pub struct MoveCommand {
    pub target: Vec2,
}
#[derive(Component)]
pub struct Movable {}

#[derive(Resource)]
struct MovementTimer(Timer);
#[derive(Component)]
pub struct MovementPath {
    pub path: Vec<PathNode>,
}

fn calculate_course_deflection(start: &NodeCoords, end: &NodeCoords) -> u32 {
    let difference: i32 = (start.h.unwrap() as i32 - end.h.unwrap() as i32).abs();
    let half_headings: i32 = (Heading::iter().len() as f32 / 2.0).ceil() as i32;
    (half_headings - (difference - half_headings).abs()) as u32
}
pub fn inertia_based_inter_cell_movement(from: NodeCoords, to: NodeCoords) -> f32 {
    let inertia: f32 = 1.0;
    let course_deflection: f32 = calculate_course_deflection(&from, &to) as f32;
    let cost: f32 =
        from.xy.as_vec2().distance(to.xy.as_vec2()).abs() + (course_deflection * inertia);
    cost
}
pub fn heuristical_distance(from: NodeCoords, to: NodeCoords) -> f32 {
    from.xy.as_vec2().distance(to.xy.as_vec2())
}
pub fn calculate_heading(from: &UVec2, to: &UVec2) -> Heading {
    let diff: IVec2 = to.as_ivec2() - from.as_ivec2();
    let heading: Heading;
    if diff.x == -1 && diff.y == 0 {
        heading = Heading::E
    } else if diff.x == -1 && diff.y == 1 {
        heading = Heading::NE
    } else if diff.x == 0 && diff.y == 1 {
        heading = Heading::N
    } else if diff.x == 1 && diff.y == 1 {
        heading = Heading::NW
    } else if diff.x == 1 && diff.y == 0 {
        heading = Heading::W
    } else if diff.x == 1 && diff.y == -1 {
        heading = Heading::SW
    } else if diff.x == 0 && diff.y == -1 {
        heading = Heading::S
    } else {
        heading = Heading::SE
    }
    heading
}
pub fn check_path_width(current: UVec2, target: UVec2, gridmap: &MovementGrid) -> bool {
    if current.x != target.x
        && current.y != target.y
        && gridmap.grid[current.x as usize][target.y as usize] != 0
        && gridmap.grid[target.x as usize][current.y as usize] != 0
    {
        return false;
    }

    true
}
pub fn get_neighbours(current: UVec2, gridmap: &MovementGrid) -> Vec<NodeCoords> {
    let mut neighbours: Vec<NodeCoords> = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            let adjacent_cell: IVec2 = IVec2 {
                x: current.x as i32 + x,
                y: current.y as i32 + y,
            };

            if adjacent_cell.x >= 0
                && (adjacent_cell.x as usize) < gridmap.grid.len()
                && adjacent_cell.y >= 0
                && (adjacent_cell.y as usize) < gridmap.grid[0].len()
                && gridmap.grid[adjacent_cell.x as usize][adjacent_cell.y as usize] == 0
                && adjacent_cell.as_uvec2() != current
                && check_path_width(current, adjacent_cell.as_uvec2(), gridmap)
            {
                neighbours.push(NodeCoords {
                    xy: UVec2 {
                        x: adjacent_cell.x as u32,
                        y: adjacent_cell.y as u32,
                    },
                    h: Some(calculate_heading(&current, &adjacent_cell.as_uvec2())),
                });
            }
        }
    }
    neighbours
}

fn move_towards(
    transform: &mut Transform,
    speed: f64,
    rotation_speed: f64,
    delta: f64,
    target: &PathNode,
) -> bool {
    let mut target_reached: bool = false;
    let target_scaled: Vec3 = Vec3 {
        x: target.xy.x,
        y: transform.translation.y,
        z: target.xy.y,
    }; // TODO make this dynamic or calculate in the reconstruct_path

    let translation_direction: Vec3 = target_scaled - transform.translation;
    let euler_rotation: (f32, f32, f32) = transform.rotation.to_euler(EulerRot::YXZ);
    let mut directional_euler_fraction: f32 =
        (Heading::iter().len() as u32 - target.h as u32) as f32 / (Heading::iter().len() as f32);
    directional_euler_fraction *= 2.0 * PI;
    directional_euler_fraction = (directional_euler_fraction + 2.0 * PI) % (2.0 * PI);
    if directional_euler_fraction > PI {
        directional_euler_fraction -= 2.0 * PI;
    }

    let target_rotation: Vec3 = Vec3 {
        x: 0.0,
        y: directional_euler_fraction,
        z: 0.0,
    };
    let rotation_direction: Vec3 = (target_rotation
        - Vec3 {
            x: euler_rotation.1,
            y: euler_rotation.0,
            z: euler_rotation.2,
        })
    .normalize_or_zero()
        * rotation_speed as f32
        * 1.
        * delta as f32;
    if rotation_direction != Vec3::ZERO {
        transform.rotate(Quat::from_euler(
            EulerRot::YXZ,
            rotation_direction.y,
            rotation_direction.x,
            rotation_direction.z,
        ));
    }
    let translation_vector: Vec3 = translation_direction.normalize() * (speed * delta) as f32;

    if translation_vector.length() >= translation_direction.length()
        || translation_direction == Vec3::ZERO
    {
        transform.translation = target_scaled;
        target_reached = true;
    } else {
        transform.translation += translation_vector;
    }
    target_reached
}
fn move_units(
    mut movables: Query<(Entity, &mut Transform, &mut MovementPath)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let speed: f64 = 1.0;
    let rotation_speed: f64 = 1.0;
    for (entity, mut transform, mut movementpath) in movables.iter_mut() {
        let node: &PathNode = match movementpath.path.last() {
            Some(n) => n,
            None => {
                commands.entity(entity).remove::<MovementPath>();
                continue;
            }
        };

        if move_towards(
            &mut transform,
            speed,
            rotation_speed,
            time.delta().as_secs_f64(),
            node,
        ) {
            commands.entity(entity).remove::<MoveCommand>();
            movementpath.path.pop();
        }
    }
}
