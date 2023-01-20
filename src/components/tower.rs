use bevy::{
    prelude::{Component, Vec3},
    reflect::Reflect,
    time::Timer,
};

#[derive(Component, Reflect, Default)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

#[derive(Component, Reflect, Default)]
pub struct Lifetime {
    pub timer: Timer,
}
