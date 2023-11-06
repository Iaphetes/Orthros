use crate::{
    ownable::Selected,
    player_controller::RayHit,
    resources::{ResourceLevel, ResourceSource},
    spawner::{EntityWrapper, UnitInformation, UnitType},
};

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

struct Collector {
    rate: f32,
    player: EntityWrapper,
}

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
    mut selected_entities: Query<(Entity, &UnitInformation), With<Selected>>,
    mut ray_hit_event: EventReader<RayHit>,
    mut resource_sources: Query<(Entity, &ResourceLevel)>,
) {
    for hit in ray_hit_event.iter() {
        for (entity, unit_information) in selected_entities.iter() {
            if unit_information.unit_type == UnitType::MiningStation {
                if let source = resource_sources.get(hit.hit_entity) {}
            }
        }
    }
}
