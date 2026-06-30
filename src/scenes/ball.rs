use bevy::color::palettes::css::GREEN;
use bevy::prelude::*;
use bevy::{
    math::Vec3,
    scene::{Scene, bsn},
    transform::components::Transform,
};

use crate::components::{Ball, Collider, Shape, Velocity};

pub fn ball(
    position: Vec3,
    size: f32,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Scene {
    let shape = meshes.add(Circle::new(size));
    let material = materials.add(ColorMaterial::from_color(GREEN));
    let velocity = Vec2::new(1f32, -2f32).normalize() * 5f32;

    bsn! {
        #Ball
        Ball
        Velocity(velocity)
        Transform { translation: position }
        Collider
        Shape::Circle(size)
        Mesh2d(shape)
        MeshMaterial2d<ColorMaterial>(material)
    }
}
