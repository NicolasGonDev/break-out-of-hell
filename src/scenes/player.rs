use bevy::prelude::*;
use bevy::{
    math::Vec3,
    scene::{Scene, bsn},
    transform::components::Transform,
};

use crate::components::{Collider, Player, Shape};

pub fn player(
    position: Vec3,
    size: Vec2,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Scene {
    let shape = meshes.add(Rectangle::new(size.x, size.y));
    let material = materials.add(ColorMaterial::from_color(Color::WHITE));

    bsn! {
        #Player
        Player
        Transform { translation: position }
        Collider
        Shape::Rectangle(size)
        Mesh2d(shape)
        MeshMaterial2d<ColorMaterial>(material)
    }
}
