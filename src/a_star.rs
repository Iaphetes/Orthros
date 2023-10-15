use crate::environment::MovementGrid;
use crate::movable::{
    get_neighbours, heuristical_distance, inertia_based_inter_cell_movement, Heading, MoveCommand,
    MovementPath, NodeCoords, PathNode, DISTANCE_FACTOR,
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

#[derive(Component)]
pub struct AStarParams {
    movement_grid: Vec<Vec<HashMap<Heading, AStarNode>>>,
    open_set: HashSet<NodeCoords>,
    target: UVec2,
    came_from: HashMap<NodeCoords, NodeCoords>,
}
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct AStarNode {
    f_score: i32,
    g_score: i32,
    came_from: Option<UVec2>,
}
fn reconstruct_path(
    came_from: &HashMap<NodeCoords, NodeCoords>,
    end: NodeCoords,
    gridmap: &MovementGrid,
) -> Vec<PathNode> {
    let mut total_path: Vec<PathNode> = vec![];

    let mut current: NodeCoords = end;
    current = came_from[&current];
    let endnode: PathNode = PathNode {
        xy: current.xy.as_vec2() - gridmap.settings.xy_offset,
        h: end.h.unwrap_or_default(),
    };

    total_path.push(endnode);
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(PathNode {
            xy: (current.xy.as_vec2() - gridmap.settings.xy_offset) * gridmap.settings.cell_size,
            h: current.h.unwrap_or_default(),
        });
    }
    total_path
}
pub fn a_star(
    movables: Query<(Entity, &mut Transform, &mut MoveCommand), Without<MovementPath>>,
    gridmap: Res<MovementGrid>,
    mut commands: Commands,
) {
    for (entity, transform, movcmd) in movables.iter() {
        if transform.translation.x == movcmd.target.x && transform.translation.y == movcmd.target.y
        {
            commands.entity(entity).remove::<MoveCommand>();
            continue;
        }
        let target: UVec2 =
            (movcmd.target / gridmap.settings.cell_size + gridmap.settings.xy_offset).as_uvec2();
        let start: UVec2 = UVec2 {
            x: (transform.translation.x / gridmap.settings.cell_size + gridmap.settings.xy_offset.x)
                as u32,
            y: (transform.translation.z / gridmap.settings.cell_size + gridmap.settings.xy_offset.y)
                as u32,
        };
        let mut movement_grid: Vec<Vec<HashMap<Heading, AStarNode>>> = vec![
            vec![
                Heading::iter()
                    .map(|x| (
                        x,
                        AStarNode {
                            f_score: -1,
                            g_score: -1,
                            came_from: None
                        }
                    ))
                    .collect();
                gridmap.grid.len()
            ];
            gridmap.grid[0].len()
        ];
        movement_grid[start.x as usize][start.y as usize]
            .get_mut(&Heading::N)
            .unwrap()
            .g_score = 0;
        commands
            .entity(entity)
            .insert(AStarParams {
                movement_grid,
                open_set: HashSet::from([NodeCoords {
                    xy: start,
                    h: Some(Heading::N),
                }]),
                came_from: HashMap::new(),
                target,
            })
            .remove::<MoveCommand>();
    }
}
pub fn calculate_a_star(
    mut movables: Query<(Entity, &mut AStarParams), Without<MovementPath>>,
    gridmap: Res<MovementGrid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut params) in movables.iter_mut() {
        let mut current: NodeCoords = NodeCoords {
            xy: UVec2::ZERO,
            h: Some(Heading::N),
        };
        let mut current_cost = 0;
        for open_cell in &params.open_set {
            let cell: &AStarNode = params.movement_grid[open_cell.xy.x as usize]
                [open_cell.xy.y as usize]
                .get(&open_cell.h.unwrap_or_default())
                .unwrap();
            let cell_f_score: i32 = cell.f_score;
            if current_cost == 0 || cell_f_score < current_cost {
                current = *open_cell;
                current_cost = cell_f_score;
            }
        }

        let current_real: Vec2 =
            (current.xy.as_vec2() - gridmap.settings.xy_offset) * gridmap.settings.cell_size;
        //        commands.spawn(PbrBundle {
        //            mesh: meshes.add(Mesh::from(shape::Plane {
        //                size: 0.5,
        //                subdivisions: 1,
        //            })),
        //            material: materials.add(Color::YELLOW.into()),
        //            transform: Transform::from_xyz(current_real.x, 0.5, current_real.y),
        //            ..default()
        //        });
        let current_node: AStarNode = params.movement_grid[current.xy.x as usize]
            [current.xy.y as usize]
            .get(&current.h.unwrap_or_default())
            .unwrap()
            .to_owned();

        if current.xy == params.target {
            let mut movementpath: MovementPath = MovementPath { path: Vec::new() };

            reconstruct_path(&params.came_from, current, &gridmap)
                .iter()
                .enumerate()
                .for_each(|(i, x)| {
                    if i != 0 {
                        movementpath.path.push(*x);
                    }
                });
            commands.entity(entity).insert(movementpath);
            commands.entity(entity).remove::<AStarParams>();
            return;
        }
        params.open_set.remove(&current);
        let neighbours = get_neighbours(current.xy, &gridmap);
        let target: UVec2 = params.target;
        for neighbour in neighbours {
            let neighbour_node: &mut AStarNode = params.movement_grid[neighbour.xy.x as usize]
                [neighbour.xy.y as usize]
                .get_mut(&neighbour.h.unwrap_or_default())
                .unwrap();
            let tentative_g_score: i32 = current_node.g_score
                + (inertia_based_inter_cell_movement(current, neighbour) * DISTANCE_FACTOR) as i32;

            if tentative_g_score < neighbour_node.g_score || neighbour_node.g_score == -1 {
                neighbour_node.g_score = tentative_g_score;
                neighbour_node.f_score = tentative_g_score
                    + (heuristical_distance(
                        neighbour,
                        NodeCoords {
                            xy: target,
                            h: None,
                        },
                    ) * DISTANCE_FACTOR) as i32;
                params.came_from.insert(neighbour, current);
                params.open_set.insert(neighbour);
            }
            println!(
                "F Score {}",
                params.movement_grid[neighbour.xy.x as usize][neighbour.xy.y as usize]
                    .get_mut(&neighbour.h.unwrap_or_default())
                    .unwrap()
                    .f_score
            );
        }
    }
}
