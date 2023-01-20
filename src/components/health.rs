use bevy::{prelude::Component, reflect::Reflect};

#[derive(Reflect, Component, Default)]
pub struct Health {
    pub value: i32,
}
