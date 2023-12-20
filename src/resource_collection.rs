use std::time::Duration;

use crate::{
    civilisation::{CivilisationBoni, CivilisationBoniMap},
    ownable::Selected,
    player_controller::RayHit,
    resources::{ResourceLevel, ResourceLevels, ResourceSource, ResourceType, ResourceUpdateEvent},
    spawner::{EntityWrapper, UnitInformation, UnitStat, UnitType},
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
    resource_entity: EntityWrapper,
    rate: f32,
    player: EntityWrapper,
    collecting: bool,
}

pub struct ResourceCollection;
#[derive(Event)]
pub struct DeselectEvent;
impl Plugin for ResourceCollection {
    fn build(&self, app: &mut App) {
        app.add_event::<RayHit>()
            .add_systems(Update, (start_collecting, collect))
            .add_event::<ResourceUpdateEvent>();
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
    for hit in ray_hit_event.read() {
        if let Ok(_) = resource_sources.get(hit.hit_entity) {
            for (entity, unit_information) in selected_entities.iter() {
                match unit_information.unit_type {
                    UnitType::MiningStation => {
                        commands.entity(entity).insert(Collector {
                            resource: ResourceType::Plotanium, //TODO make adaptive
                            resource_entity: EntityWrapper {
                                entity: hit.hit_entity,
                            },
                            rate: 24.1,

                            player: EntityWrapper {
                                entity: main_player_entity,
                            },
                            collecting: false,
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
    mut collectors: Query<(Entity, &mut Collector, &Transform, &UnitInformation)>,
    mut resource_levels: Query<&mut ResourceLevels>,
    resource_location: Query<&Transform, With<ResourceLevel>>,
    mut stopwatch: Local<Stopwatch>,
    mut resource_update_events: EventWriter<ResourceUpdateEvent>,
    mut commands: Commands,
) {
    stopwatch.tick(time.delta());
    if stopwatch.elapsed().as_secs() >= 1 {
        stopwatch.reset();
        for (entity, mut collector, collector_transform, unit_information) in collectors.iter_mut()
        {
            let mut dist = 0.0;
            if let Ok(resource_transform) = resource_location.get(collector.resource_entity.entity)
            {
                dist = collector_transform
                    .translation
                    .distance(resource_transform.translation);
            }

            let mut max_mining_dist: f32 = 0.0;
            for stat in &unit_information.stats.0 {
                if let UnitStat::MaxMiningDist(m) = stat {
                    max_mining_dist = *m;
                }
            }
            if collector.collecting && max_mining_dist < dist {
                commands.entity(entity).remove::<Collector>();
            } else if !collector.collecting && max_mining_dist > dist {
                collector.collecting = true;
            }
            println!("dist: {dist}, max_dist: {max_mining_dist}");
            if collector.collecting {
                match resource_levels.get_mut(collector.player.entity) {
                    Ok(mut resource_level) => {
                        if let Some(mut resource) = resource_level.0.get_mut(&collector.resource) {
                            *resource += collector.rate as i32;

                            resource_update_events.send(ResourceUpdateEvent(ResourceLevel {
                                resource_type: ResourceType::Plotanium,
                                amount: *resource,
                            }));
                        }
                    }
                    Err(_) => {
                        println!("Could not find player")
                    }
                }
            }
        }
    }
}
