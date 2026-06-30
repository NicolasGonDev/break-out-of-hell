use bevy::{
    ecs::component::Component,
    math::Vec2,
    prelude::{Deref, DerefMut},
};

#[derive(Component, Default, Clone, Deref, DerefMut, Debug)]
pub struct Velocity(pub Vec2);
