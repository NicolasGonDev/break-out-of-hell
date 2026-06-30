use bevy::{
    ecs::{
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query},
    },
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    transform::components::Transform,
};

use crate::{
    components::{Ball, Brick, Collider, Shape, Velocity},
    enums::Collision,
    events::BallCollided,
};

pub fn check_for_collisions(
    mut commands: Commands,
    ball_query: Query<(&mut Velocity, &Transform, &Shape), With<Ball>>,
    collider_query: Query<
        (Entity, &Transform, &Shape, Option<&Brick>),
        (With<Collider>, Without<Ball>),
    >,
) {
    for (mut ball_velocity, ball_transform, ball_shape) in ball_query {
        for (collider_entity, collider_transform, collider_shape, _maybe_brick) in &collider_query {
            if let Shape::Circle(ball_size) = ball_shape {
                let pos = collider_transform.translation.truncate();

                if let Shape::Rectangle(collider_size) = collider_shape {
                    let collision = ball_collision(
                        BoundingCircle::new(ball_transform.translation.truncate(), *ball_size),
                        Aabb2d::new(pos, *collider_size / 2f32),
                    );

                    if let Some(collision) = collision {
                        // Trigger observers of the "BallCollided" event
                        commands.trigger(BallCollided {
                            entity: collider_entity,
                        });

                        // Bricks should be despawned and increment the scoreboard on collision
                        // if maybe_brick.is_some() {
                        //     commands.entity(collider_entity).despawn();
                        //     **score += 1;
                        // }

                        // Reflect the ball's velocity when it collides
                        let mut reflect_x = false;
                        let mut reflect_y = false;

                        // Reflect only if the velocity is in the opposite direction of the collision
                        // This prevents the ball from getting stuck inside the bar
                        match collision {
                            Collision::Left => reflect_x = ball_velocity.x > 0.0,
                            Collision::Right => reflect_x = ball_velocity.x < 0.0,
                            Collision::Top => reflect_y = ball_velocity.y < 0.0,
                            Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                        }

                        // Reflect velocity on the x-axis if we hit something on the x-axis
                        if reflect_x {
                            ball_velocity.x = -ball_velocity.x;
                        }

                        // Reflect velocity on the y-axis if we hit something on the y-axis
                        if reflect_y {
                            ball_velocity.y = -ball_velocity.y;
                        }
                    }
                }
            }
        }
    }
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
