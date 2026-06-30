use bevy::color::palettes::css::BLUE;
use bevy::prelude::*;
use bevy::{
    math::Vec3,
    scene::{Scene, bsn},
    transform::components::Transform,
};

use crate::components::{Collider, Shape, Wall};

pub fn wall(
    position: Vec3,
    size: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Scene {
    let shape = meshes.add(Rectangle::new(size.x, size.y));
    let material = materials.add(ColorMaterial::from_color(BLUE));

    bsn! {
        #Wall
        Wall
        Transform { translation: position }
        Collider
        Shape::Rectangle(size)
        Mesh2d(shape)
        MeshMaterial2d<ColorMaterial>(material)
    }
}
