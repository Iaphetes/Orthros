use bevy::{prelude::*, utils::HashMap};
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum ResourceType {
    Plotanium,
}
#[derive(Component)]
pub struct ResourceLevel {
    pub resource_type: ResourceType,
    pub resource_amount: i32,
}
#[derive(Component)]
pub struct ResourceStockpiles(pub HashMap<ResourceType, i32>);

impl ResourceStockpiles {
    pub fn get(&self, resource_type: &ResourceType) -> Option<&i32> {
        self.0.get(resource_type)
    }
}

#[derive(Component)]
pub struct ResourceSource;
