use bevy::prelude::*;
use bevy::{
    math::Vec3,
    scene::{Scene, bsn},
    transform::components::Transform,
};

use crate::components::{Brick, Collider, Health, Shape};
use crate::events::BallCollided;
use crate::resources::BrickMaterials;

pub fn brick(
    position: Vec3,
    size: Vec2,
    name: String,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &Res<BrickMaterials>,
) -> impl Scene {
    let shape = meshes.add(Rectangle::new(size.x, size.y));
    let material = materials.get(3).unwrap().clone();

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
    mut query: Query<(&mut Health, &mut MeshMaterial2d<ColorMaterial>), With<Brick>>,
    mut commands: Commands,
    materials: Res<BrickMaterials>,
) {
    if let Ok((mut health, mut material)) = query.get_mut(event.entity) {
        **health = (**health).saturating_sub(1);
        *material = MeshMaterial2d(materials.get((**health) as usize).unwrap().clone());
        if **health == 0 {
            commands.entity(event.entity).despawn();
        }
    }
}
