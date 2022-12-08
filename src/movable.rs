use bevy::ecs::component::Component;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::transform::components::Transform;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Component)]
pub struct MoveTarget {
    pub target: Vec3,
}
const DISTANCE_FACTOR: f32 = 100.0;

struct NodeCoords {
    xy: UVec2,
    h: Heading,
}

#[derive(Eq, PartialEq, Hash, Clone, EnumIter, Debug)]
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
struct MovementGrid {
    grid: Vec<Vec<u8>>,
}
#[derive(Component)]
struct MoveCommand {
    target: Vec2,
    path: Vec<NodeCoords>,
}
#[derive(Component)]
struct Movable {}

#[derive(Resource)]
struct GridSettings {
    cell_size: f32,
    grid_width: u32,
    grid_height: u32,
    x_y_offset: Vec2,
    density: f64, // TODO put into map generation
}

#[derive(Resource)]
struct MovementTimer(Timer);
pub fn move_units(
    mut movable_units: Query<(Entity, &mut Transform, &MoveTarget)>,
    mut commands: Commands,
) {
    for (mut entity, mut transform, movetarget) in movable_units.iter_mut() {
        if movetarget.target != transform.translation {
            let rotation_xz: f32 = Vec2 {
                x: movetarget.target.x - transform.translation.x,
                y: movetarget.target.z - transform.translation.z,
            }
            .angle_between(Vec2 { x: 0.0, y: 1.0 });
            println!("{:?}", rotation_xz);
            transform.rotation = Quat::from_rotation_y(rotation_xz);
            transform.translation = movetarget.target;
        }
        commands.entity(entity).remove::<MoveTarget>();
    }
}
