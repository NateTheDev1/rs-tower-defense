use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, Reflect, Default)]
pub struct Target {
    pub speed: f32,
}
