use bevy::{prelude::*, utils::HashMap};
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ResourceType {
    Plotanium,
}
#[derive(Event)]
pub struct ResourceUpdateEvent(pub ResourceLevel);
#[derive(Component)]
pub struct ResourceLevel {
    pub resource_type: ResourceType,
    pub amount: i32,
}
#[derive(Component)]
pub struct ResourceLevels(pub HashMap<ResourceType, i32>);
#[derive(Component)]
pub struct ResourceSource;
