use std::ops::Sub;

use bevy::color::palettes::css::PINK;
use bevy::prelude::*;
use bevy::{
    math::Vec3,
    scene::{Scene, bsn},
    transform::components::Transform,
};

use crate::components::{Brick, Collider, Health, Shape};
use crate::events::BallCollided;

pub fn brick(
    position: Vec3,
    size: Vec2,
    name: String,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> impl Scene {
    let shape = meshes.add(Rectangle::new(size.x, size.y));
    let material = materials.add(ColorMaterial::from_color(PINK));

    bsn! {
        Name(name)
        Brick
        Transform { translation: position }
        Collider
        Shape::Rectangle(size)
        Mesh2d(shape)
        MeshMaterial2d<ColorMaterial>(material)
        Health(4)
        on(take_damage)
    }
}

fn take_damage(
    event: On<BallCollided>,
    mut query: Query<(&mut Health, &Name), With<Brick>>,
    mut commands: Commands,
) {
    if let Ok((mut health, name)) = query.get_mut(event.entity) {
        **health = (**health).saturating_sub(1);
        let h = **health;
        info!("{name} GOT HIT! {h} left!");
        if **health == 0 {
            commands.entity(event.entity).despawn();
        }
    }
}
