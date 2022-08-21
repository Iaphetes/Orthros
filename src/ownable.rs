use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Selectable {
    pub selected : bool
}
#[derive(Component)]
pub struct SelectionCircle {}


