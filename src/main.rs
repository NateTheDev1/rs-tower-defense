mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

use bevy::{
    prelude::{
        default, shape, App, AssetServer, Assets, Camera3dBundle, ClearColor, Color, Commands,
        Handle, Mesh, Name, PbrBundle, PluginGroup, PointLight, PointLightBundle, Res, ResMut,
        Resource, StandardMaterial, Transform, Vec3,
    },
    scene::Scene,
    time::Timer,
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub fn spawn_lighting(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

pub fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..Default::default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(-4.0, 0.2, 1.5),
            ..Default::default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Health { value: 3 })
        .insert(Name::new("Ground"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(Tower {
            bullet_offset: Vec3::default(),
            shooting_timer: Timer::from_seconds(1.0, bevy::time::TimerMode::Repeating),
        })
        .insert(Name::new("Tower"));
}

#[derive(Resource)]
pub struct GameAssets {
    pub bullet_scene: Handle<Scene>,
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

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
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_lighting)
        .add_startup_system(spawn_camera)
        .add_startup_system(asset_loading)
        .run();
}
