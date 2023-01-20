use bevy::{
    prelude::{Component, Vec3},
    reflect::Reflect,
};

#[derive(Reflect, Component, Default)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}
