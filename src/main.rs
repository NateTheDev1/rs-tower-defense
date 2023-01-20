pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use components::{
    bullet::Bullet,
    health::Health,
    target::Target,
    tower::{Lifetime, Tower},
};
use systems::{
    asset_loading::asset_loading,
    bullet_collision::bullet_collision,
    move_bullets::move_bullets,
    move_targets::move_targets,
    spawn_basic_scene::spawn_basic_scene,
    spawn_camera::spawn_camera,
    spawn_lighting::spawn_lighting,
    target_death::target_death,
    tower_shooting::{bullet_despawn, tower_shooting},
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
        .add_plugin(WorldInspectorPlugin)
        .register_type::<Tower>()
        .register_type::<Bullet>()
        .register_type::<Lifetime>()
        .register_type::<Target>()
        .register_type::<Health>()
        .add_startup_system(asset_loading)
        .add_startup_system(spawn_lighting)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(tower_shooting)
        .add_system(move_bullets)
        .add_system(move_targets)
        .add_system(bullet_despawn)
        .add_system(target_death)
        .add_system(bullet_collision)
        .run();
}
