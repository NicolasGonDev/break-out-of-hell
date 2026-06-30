use bevy::ecs::component::Component;

use crate::components::Shape;

#[derive(Component, Default, Clone)]
#[require(Shape)]
pub struct Collider;
