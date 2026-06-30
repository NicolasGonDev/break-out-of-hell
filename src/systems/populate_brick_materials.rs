use bevy::{
    asset::Assets, color::palettes::css::*, ecs::system::ResMut, sprite_render::ColorMaterial,
};

use crate::resources::BrickMaterials;

pub fn populate_brick_materials(
    mut brick_materials: ResMut<BrickMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    brick_materials.push(materials.add(ColorMaterial::from_color(BLACK)));
    brick_materials.push(materials.add(ColorMaterial::from_color(PINK)));
    brick_materials.push(materials.add(ColorMaterial::from_color(HOT_PINK)));
    brick_materials.push(materials.add(ColorMaterial::from_color(DEEP_PINK)));
}
