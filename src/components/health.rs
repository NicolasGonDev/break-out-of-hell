use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

#[derive(Component, Default, Clone, Deref, DerefMut, Debug)]
pub struct Health(pub u32);
