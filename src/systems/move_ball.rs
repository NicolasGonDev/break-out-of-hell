use bevy::{
    ecs::{query::With, system::Query},
    transform::components::Transform,
};

use crate::components::{Ball, Velocity};

pub fn move_ball(ball_trs: Query<(&mut Transform, &Velocity), With<Ball>>) {
    for (mut transform, velocity) in ball_trs {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}
