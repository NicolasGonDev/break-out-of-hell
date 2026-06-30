use bevy::ecs::{entity::Entity, event::EntityEvent};

#[derive(EntityEvent)]
pub struct BallCollided {
    pub entity: Entity,
}
