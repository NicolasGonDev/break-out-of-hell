use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Clone)]
pub enum Shape {
    Rectangle(Vec2),
    Circle(f32),
}

impl Default for Shape {
    fn default() -> Self {
        Self::Rectangle(Vec2::ONE)
    }
}

impl Shape {
    pub const fn default_rectangle() -> Self {
        Self::Rectangle(Vec2::ONE)
    }

    pub const fn default_circle() -> Self {
        Self::Circle(1f32)
    }
}
