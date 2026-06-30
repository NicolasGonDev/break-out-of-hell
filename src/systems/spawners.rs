use bevy::{
    asset::Assets,
    camera::Camera2d,
    ecs::name::Name,
    ecs::system::{Commands, ResMut},
    math::Vec2,
    math::Vec3,
    mesh::Mesh,
    scene::CommandsSceneExt,
    sprite_render::ColorMaterial,
    transform::components::Transform,
};

use crate::scenes::player;
use crate::scenes::wall;
use crate::scenes::{ball, brick};

pub fn spawn_ball(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_scene(ball(
        Vec3::new(0f32, 300f32, 5f32),
        12f32,
        meshes,
        materials,
    ));
}

pub fn spawn_ball2(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_scene(ball(
        Vec3::new(100f32, -300f32, 5f32),
        12f32,
        meshes,
        materials,
    ));
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..4usize {
        commands.spawn_scene(wall(
            Vec3::new(WALL_POS[i].0, WALL_POS[i].1, 6f32),
            Vec2::new(WALL_SIZES[i].0, WALL_SIZES[i].1),
            &mut meshes,
            &mut materials,
        ));
    }
}

pub fn spawn_bricks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let width: i32 = 900;
    let height: i32 = 300;
    let offset_y: i32 = 100;

    for y in 0..8i32 {
        for x in 0..16i32 {
            commands.spawn_scene(brick(
                Vec3::new(
                    ((x * (width / 16)) - (width / 2)) as f32,
                    (((y * (height / 8)) - (height / 2)) + offset_y) as f32,
                    6f32,
                ),
                Vec2::new(8f32, 8f32),
                format!("Brick {x} {y}"),
                &mut meshes,
                &mut materials,
            ));
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_scene(player(
        Vec3::new(0f32, -300f32, 5f32),
        Vec2::new(200f32, 16f32),
        meshes,
        materials,
    ));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d::default(),
        Transform::from_xyz(0f32, 0f32, -5f32).looking_at(Vec3::Z, Vec3::Y),
    ));
}

// top, right, bottom, left
const WALL_POS: [(f32, f32); 4] = [
    (0f32, -360f32),
    (640f32, 0f32),
    (0f32, 360f32),
    (-640f32, 0f32),
];
const WALL_SIZES: [(f32, f32); 4] = [
    (1280f32, 8f32),
    (8f32, 720f32),
    (1280f32, 8f32),
    (8f32, 720f32),
];
