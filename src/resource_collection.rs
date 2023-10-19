use crate::{ownable::Selected, player_controller::RayHit};

use bevy::prelude::*;
#[repr(u8)]
pub enum RenderLayerMap {
    General = 0,
    Main = 1,
    Minimap = 2,
}
use bevy_rapier3d::prelude::*;
pub struct ResourceCollection;
#[derive(Event)]
pub struct DeselectEvent;
impl Plugin for ResourceCollection {
    fn build(&self, app: &mut App) {
        app.add_event::<RayHit>()
            .add_systems(Update, (start_collecting));
    }
}

fn start_collecting(
    mut selected_entities: Query<(Entity, &Selected)>,
    mut ray_hit_event: EventReader<RayHit>,
) {
    for hit in ray_hit_event.iter() {}
}
