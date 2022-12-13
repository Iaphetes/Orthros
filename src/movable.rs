use bevy::ecs::component::Component;
use bevy::math::Vec3;
use bevy::prelude::*;

use bevy::transform::components::Transform;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::environment::{GridSettings, MovementGrid};

pub struct UnitMovement;

impl Plugin for UnitMovement {
    fn build(&self, app: &mut App) {
        app.add_system(calculate_a_star)
            .add_system(move_units)
            .insert_resource(MovementTimer(Timer::new(
                Duration::from_millis(1500),
                TimerMode::Repeating,
            )));
    }
}
// #[derive(Component)]
// pub struct MoveTarget {
//     pub target: Vec3,
// }
const DISTANCE_FACTOR: f32 = 100.0;
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct NodeCoords {
    xy: UVec2,
    h: Option<Heading>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, EnumIter, Debug)]
enum Heading {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
#[derive(Component)]
pub struct MoveCommand {
    pub target: Vec2,
    pub path: Vec<NodeCoords>,
}
#[derive(Component)]
struct Movable {}

#[derive(Resource)]
struct MovementTimer(Timer);
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct AStarNode {
    f_score: i32,
    g_score: i32,
    came_from: Option<UVec2>,
}
// pub fn move_units(
//     mut movable_units: Query<(Entity, &mut Transform, &MoveTarget)>,
//     mut commands: Commands,
// ) {
//     for (mut entity, mut transform, movetarget) in movable_units.iter_mut() {
//         if movetarget.target != transform.translation {
//             let rotation_xz: f32 = Vec2 {
//                 x: movetarget.target.x - transform.translation.x,
//                 y: movetarget.target.z - transform.translation.z,
//             }
//             .angle_between(Vec2 { x: 0.0, y: 1.0 });
//             println!("{:?}", rotation_xz);
//             transform.rotation = Quat::from_rotation_y(rotation_xz);
//             transform.translation = movetarget.target;
//         }
//         commands.entity(entity).remove::<MoveTarget>();
//     }
// }
fn calculate_a_star(
    mut movables: Query<(Entity, &mut Transform, &mut MoveCommand), Without<Movable>>,
    mut gridmap_q: Query<&mut MovementGrid>,
    mut commands: Commands,
) //-> Option<Vec<UVec2>>
{
    for (entity, transform, mut movcmd) in movables.iter_mut() {
        println!("calculating a*");
        if transform.translation.x == movcmd.target.x && transform.translation.y == movcmd.target.y
        {
            commands.entity(entity).remove::<MoveCommand>();
            continue;
        }
        println!("Current position {}", transform.translation);
        println!("Target position {}", movcmd.target);

        println!("{}", gridmap_q.into_iter().len());
        match gridmap_q.get_single_mut() {
            Ok(gridmap) => {
                let target: UVec2 = movcmd.target.as_uvec2();

                let mut movement_grid: Vec<Vec<HashMap<Heading, AStarNode>>> = vec![
                        vec![Heading::iter()
                            .map(|x| (
                                x.clone(),
                                AStarNode {
                                    f_score: -1,
                                    g_score: -1,
                                    came_from: None
                                }
                            ))
                            .into_iter()
                            .collect();
                        gridmap.grid.len()];
                gridmap.grid[0].len()
                    ];
                // println!("X_Length: {}, Y_Length: {}, Headings: {}", gridmap.grid.len(), gridmap)
                let mut came_from: HashMap<NodeCoords, NodeCoords> = HashMap::new();
                let mut open_set: HashSet<NodeCoords> = HashSet::from([NodeCoords {
                    xy: UVec2 {
                        x: transform.translation.x.floor() as u32,
                        y: transform.translation.y.floor() as u32,
                    },
                    h: Some(Heading::N),
                }]);
                movement_grid[transform.translation.x.floor() as usize]
                    [transform.translation.y.floor() as usize]
                    .get_mut(&Heading::N)
                    .unwrap()
                    .g_score = 0;
                while !open_set.is_empty() {
                    let mut current: NodeCoords = NodeCoords {
                        xy: UVec2::ZERO,
                        h: Some(Heading::N),
                    };

                    let mut current_cost = 0;
                    for open_cell in open_set.clone() {
                        let cell: &AStarNode = movement_grid[open_cell.xy.x as usize]
                            [open_cell.xy.y as usize]
                            .get_mut(&open_cell.h.unwrap_or(Heading::N))
                            .unwrap();
                        let cell_f_score: i32 = cell.f_score;
                        if current_cost == 0 || cell_f_score < current_cost {
                            current = open_cell.clone();
                            current_cost = cell_f_score;
                        }
                    }

                    let current_node: AStarNode = movement_grid[current.xy.x as usize]
                        [current.xy.y as usize]
                        .get(&current.h.unwrap_or(Heading::N))
                        .unwrap()
                        .to_owned();
                    if current.xy == movcmd.target.as_uvec2() {
                        for node in reconstruct_path(&came_from, current) {
                            if !(node.xy.x == transform.translation.x.floor() as u32
                                && node.xy.y == transform.translation.y.floor() as u32)
                                && node.xy != movcmd.target.as_uvec2()
                            {
                                movcmd.path.push(node);
                                println!("Node {:?}", node);
                            }

                            commands.entity(entity).insert(Movable {});
                        }
                        return;
                    }
                    open_set.remove(&current);
                    let neighbours = get_neighbours(current.xy, &gridmap);
                    // println!("Current: {:?}", current);
                    for neighbour in neighbours {
                        // println!("{:?}", neighbour);
                        let mut neighbour_node: &mut AStarNode = movement_grid
                            [neighbour.xy.x as usize][neighbour.xy.y as usize]
                            .get_mut(&neighbour.h.unwrap_or(Heading::N))
                            .unwrap();
                        let tentative_g_score: i32 = current_node.g_score
                            + (inertia_based_inter_cell_movement(
                                current.clone(),
                                neighbour.clone(),
                            ) * DISTANCE_FACTOR) as i32;

                        if tentative_g_score < neighbour_node.g_score
                            || neighbour_node.g_score == -1
                        {
                            neighbour_node.g_score = tentative_g_score;
                            neighbour_node.f_score = tentative_g_score
                                + (heuristical_distance(
                                    neighbour,
                                    NodeCoords {
                                        xy: target,
                                        h: None,
                                    },
                                ) * DISTANCE_FACTOR) as i32;
                            came_from.insert(neighbour, current);
                            open_set.insert(neighbour);
                        }
                    }
                }
            }
            Err(error) => {
                println!("Cannot comply{:?}", error);
            }
        }
    }
    return; // None;
}
fn reconstruct_path(
    came_from: &HashMap<NodeCoords, NodeCoords>,
    end: NodeCoords,
) -> Vec<NodeCoords> {
    let mut total_path: Vec<NodeCoords> = vec![end.clone()];

    let mut current: NodeCoords = end;
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current.clone());
        println!("{:?}", current);
    }
    println!("{:?}", total_path);
    return total_path;
}
fn calculate_base_inertia(heading_in: Heading, heading_out: Heading) -> u32 {
    // println!("Heading in {:?}, Heading out {:?}", heading_in, heading_out);
    let mut penalty: u32 = 0;
    let difference: i32 = (heading_out as i32 - heading_in as i32).abs();
    let half_headings: i32 = (Heading::iter().len() as f32 / 2.0).ceil() as i32;
    // println!("difference {} half_headings {}", difference, half_headings);
    // if difference.abs() > half_headings {
    penalty = (half_headings - (difference - half_headings).abs()) as u32;
    // }
    // println!("penalty {}", penalty);
    return penalty;
}
fn inertia_based_inter_cell_movement(from: NodeCoords, to: NodeCoords) -> f32 {
    let inertia: f32 = 0.0;
    let penalty: f32 =
        calculate_base_inertia(from.h.unwrap_or(Heading::N), to.h.unwrap_or(Heading::N)) as f32;

    let cost: f32 = from.xy.as_vec2().distance(to.xy.as_vec2()).abs() + penalty;
    // println!(
    //     "from {:?} to {:?} penalty {:?}, cost {:?}",
    //     from, to, penalty, cost
    // );
    return cost;
}
fn heuristical_distance(from: NodeCoords, to: NodeCoords) -> f32 {
    return from.xy.as_vec2().distance(to.xy.as_vec2());
}
fn calculate_heading(from: &UVec2, to: &UVec2) -> Heading {
    let diff: IVec2 = from.as_ivec2() - to.as_ivec2();
    let heading: Heading;
    if diff.x == 1 && diff.y == 0 {
        heading = Heading::N
    } else if diff.x == 1 && diff.y == 1 {
        heading = Heading::NE
    } else if diff.x == 0 && diff.y == 1 {
        heading = Heading::E
    } else if diff.x == -1 && diff.y == 1 {
        heading = Heading::SE
    } else if diff.x == -1 && diff.y == 0 {
        heading = Heading::S
    } else if diff.x == -1 && diff.y == -1 {
        heading = Heading::SW
    } else if diff.x == 0 && diff.y == -1 {
        heading = Heading::W
    } else {
        heading = Heading::NW
    }
    return heading;
}
fn check_path_width(current: UVec2, target: UVec2, gridmap: &MovementGrid) -> bool {
    if current.x != target.x && current.y != target.y {
        if gridmap.grid[current.x as usize][target.y as usize] != 0
            && gridmap.grid[target.x as usize][current.y as usize] != 0
        {
            println!("current {} neighbour {}", current, target);
            return false;
        }
    }
    return true;
}
fn get_neighbours(current: UVec2, gridmap: &MovementGrid) -> Vec<NodeCoords> {
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
                && check_path_width(current, adjacent_cell.as_uvec2(), &gridmap)
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
    return neighbours;
}

fn move_units(
    mut movables: Query<(Entity, &mut Transform, &mut MoveCommand), With<Movable>>,
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut commands: Commands,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        timer.0.set_duration(Duration::from_millis(150));
        for (entity, mut transform, mut movcmd) in movables.iter_mut() {
            let node: NodeCoords;
            match movcmd.path.pop() {
                Some(n) => node = n,
                None => {
                    commands.entity(entity).remove::<MoveCommand>();
                    commands.entity(entity).remove::<Movable>();
                    continue;
                }
            }
            transform.translation = Vec3::new(
                node.xy.x as f32 * 0.2,
                transform.translation.y,
                node.xy.y as f32 * 0.2,
            );
        }
    }
}
