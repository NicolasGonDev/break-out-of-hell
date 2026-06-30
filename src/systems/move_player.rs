use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec3,
    transform::components::Transform,
};

use crate::components::Player;

pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    player_trs: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_trs {
        if keys.pressed(KeyCode::KeyD) {
            transform.translation += Vec3::new(-5f32, 0f32, 0f32);
        }
        if keys.pressed(KeyCode::KeyA) {
            transform.translation += Vec3::new(5f32, 0f32, 0f32);
        }
    }
}
