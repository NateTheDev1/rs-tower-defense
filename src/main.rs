mod bullet;
mod target;
mod tower;

use bevy_mod_picking::{
    DefaultPickingPlugins, Highlighting, PickableBundle, PickingCameraBundle, Selection,
};
pub use bullet::*;
pub use target::*;
pub use tower::*;

use bevy::{
    ecs::query::QuerySingleError,
    pbr::NotShadowCaster,
    prelude::{
        default, shape, App, AssetServer, Assets, BuildChildren, ButtonBundle, Camera3d,
        Camera3dBundle, Changed, ClearColor, Color, Commands, Component, DespawnRecursiveExt,
        Entity, Handle, Input, KeyCode, Mesh, Name, NodeBundle, PbrBundle, PluginGroup, PointLight,
        PointLightBundle, Query, Res, ResMut, Resource, SpatialBundle, StandardMaterial,
        StartupStage, Transform, Vec3, With,
    },
    scene::{Scene, SceneBundle},
    time::{Time, Timer, TimerMode},
    ui::{AlignSelf, Interaction, JustifyContent, Size, Style, UiRect, Val},
    window::{WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_lighting)
        .add_startup_system(spawn_camera)
        .add_system(create_ui_on_selection)
        .add_system(camera_controls)
        .add_system(what_is_selected)
        .add_system(tower_button_clicked)
        .run();
}

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
    game_assets: Res<GameAssets>,
) {
    commands
        .spawn(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.4, 2.5),
            ..Default::default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-4.0, 0.4, 2.5),
            ..Default::default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3 })
        .insert(Name::new("Target"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.3, 0.4, 0.3).into()),
            ..Default::default()
        })
        .insert(Health { value: 3 })
        .insert(Name::new("Ground"));

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());

    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());

    for i in 0..5 {
        commands
            .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                0.0 + 0.5 + i as f32,
                0.8,
                0.0,
            )))
            .insert(Name::new("Tower_Base"))
            .insert(meshes.add(shape::Capsule::default().into()))
            .insert(Highlighting {
                initial: default_collider_color.clone(),
                hovered: Some(selected_collider_color.clone()),
                pressed: Some(selected_collider_color.clone()),
                selected: Some(selected_collider_color.clone()),
            })
            .insert(default_collider_color.clone())
            .insert(NotShadowCaster)
            .insert(PickableBundle::default())
            .with_children(|commands| {
                commands.spawn(SceneBundle {
                    scene: game_assets.tower_base_scene.clone(),
                    transform: Transform::from_xyz(0.0, -0.8, 0.0),
                    ..Default::default()
                });
            });
    }
}

fn what_is_selected(selection: Query<(&Name, &Selection)>) {
    for (name, selection) in &selection {
        if selection.selected() {
            println!("Selected: {}", name);
        }
    }
}

#[derive(Resource)]
pub struct GameAssets {
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
    potato_tower_scene: Handle<Scene>,
    potato_scene: Handle<Scene>,
    cabbage_tower_scene: Handle<Scene>,
    cabbage_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        potato_tower_scene: assets.load("PotatoTower.glb#Scene0"),
        potato_scene: assets.load("Potato.glb#Scene0"),
        cabbage_tower_scene: assets.load("CabbageTower.glb#Scene0"),
        cabbage_scene: assets.load("Cabbage.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
}

pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PickingCameraBundle::default());
}

pub fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;

    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = 3.0;
    let rotate_speed = 0.3;

    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }

    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}

fn tower_button_clicked(
    interaction: Query<(&Interaction, &TowerType), Changed<Interaction>>,
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    assets: Res<GameAssets>,
) {
    for (interaction, tower_type) in &interaction {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selection {
                if selection.selected() {
                    commands.entity(entity).despawn_recursive();

                    spawn_tower(&mut commands, &assets, transform.translation, *tower_type);
                }
            }
        }
    }
}

#[derive(Component)]
pub struct TowerUIRoot;

#[derive(Component, Clone, Copy, Debug)]
pub enum TowerType {
    Tomato,
    Potato,
    Cabbage,
}

impl TowerType {
    pub fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        match self {
            TowerType::Tomato => (
                assets.tomato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Potato => (
                assets.potato_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
            TowerType::Cabbage => (
                assets.cabbage_tower_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.8, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                },
            ),
        }
    }

    pub fn get_bullet(&self, direction: Vec3, assets: &GameAssets) -> (Handle<Scene>, Bullet) {
        match self {
            TowerType::Tomato => (
                assets.tomato_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                },
            ),
            TowerType::Potato => (
                assets.potato_scene.clone(),
                Bullet {
                    direction,
                    speed: 6.5,
                },
            ),
            TowerType::Cabbage => (
                assets.cabbage_scene.clone(),
                Bullet {
                    direction,
                    speed: 1.5,
                },
            ),
        }
    }
}

fn create_ui_on_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selections: Query<&Selection>,
    root: Query<Entity, With<TowerUIRoot>>,
) {
    let at_least_one_selected = selections.iter().any(|selection| selection.selected());
    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        }
        //No root exist
        Err(QuerySingleError::NoEntities(..)) => {
            if at_least_one_selected {
                create_ui(&mut commands, &asset_server);
            }
        }
        _ => unreachable!("Too many ui tower roots!"),
    }
}

pub fn create_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let towers = [TowerType::Tomato, TowerType::Potato, TowerType::Cabbage];

    let button_icons = [
        asset_server.load("tomato_tower.png"),
        asset_server.load("potato_tower.png"),
        asset_server.load("cabbage_tower.png"),
    ];

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(TowerUIRoot)
        .with_children(|commands| {
            for i in 0..3 {
                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                            align_self: AlignSelf::FlexEnd,
                            margin: UiRect::all(Val::Percent(2.0)),
                            ..default()
                        },
                        image: button_icons[i].clone().into(),
                        ..default()
                    })
                    .insert(towers[i]);
            }
        });
}
