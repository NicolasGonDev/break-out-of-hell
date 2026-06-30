use bevy::{
    asset::Handle,
    ecs::resource::Resource,
    prelude::{Deref, DerefMut},
    sprite_render::ColorMaterial,
};

#[derive(Resource, DerefMut, Deref, Debug)]
pub struct BrickMaterials(pub Vec<Handle<ColorMaterial>>);

impl BrickMaterials {
    pub fn new() -> Self {
        BrickMaterials(Vec::new())
    }
}
