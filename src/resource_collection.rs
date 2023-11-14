use std::time::Duration;

use crate::{
    ownable::Selected,
    player_controller::RayHit,
    resources::{ResourceLevel, ResourceLevels, ResourceSource, ResourceType},
    spawner::{EntityWrapper, UnitInformation, UnitType},
    ActivePlayer, PlayerInfo,
};

use bevy::{prelude::*, time::Stopwatch, transform::commands};
use bevy_rapier3d::prelude::*;

#[derive(Resource)]
struct CollectionTick {
    time: Stopwatch,
}
#[derive(Component)]
struct Collector {
    resource: ResourceType,
    rate: f32,
    player: EntityWrapper,
}

pub struct ResourceCollection;
#[derive(Event)]
pub struct DeselectEvent;
impl Plugin for ResourceCollection {
    fn build(&self, app: &mut App) {
        app.add_event::<RayHit>()
            .add_systems(Update, (start_collecting, collect));
    }
}

fn start_collecting(
    mut commands: Commands,
    mut selected_entities: Query<(Entity, &UnitInformation), With<Selected>>,
    mut ray_hit_event: EventReader<RayHit>,
    mut resource_sources: Query<(Entity, &ResourceLevel)>,
    main_player: Query<Entity, With<ActivePlayer>>,
) {
    let main_player_entity: Entity = main_player.get_single().unwrap();
    for hit in ray_hit_event.iter() {
        if let source = resource_sources.get(hit.hit_entity) {
            for (entity, unit_information) in selected_entities.iter() {
                match unit_information.unit_type {
                    UnitType::MiningStation => {
                        commands.entity(entity).insert(Collector {
                            resource: ResourceType::Plotanium, //TODO make adaptive
                            rate: 24.1,
                            player: EntityWrapper {
                                entity: main_player_entity,
                            },
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn collect(
    time: Res<Time>,
    collectors: Query<&Collector>,
    mut resource_levels: Query<&mut ResourceLevels>,
    mut stopwatch: Local<Stopwatch>,
) {
    stopwatch.tick(time.delta());
    if stopwatch.elapsed().as_secs() >= 1 {
        stopwatch.reset();
        for collector in collectors.iter() {
            match resource_levels.get_mut(collector.player.entity) {
                Ok(mut resource_level) => {
                    if let Some(mut resource) = resource_level.0.get_mut(&collector.resource) {
                        *resource += collector.rate as i32;
                    }
                }
                Err(_) => {
                    println!("Could not find player")
                }
            }
        }
    }
}
