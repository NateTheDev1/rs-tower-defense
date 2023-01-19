mod systems;

use bevy::prelude::*;
use systems::{
    spawn_basic_scene::spawn_basic_scene, spawn_camera::spawn_camera,
    spawn_lighting::spawn_lighting,
};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Tower Defense".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_lighting)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .run();
}
