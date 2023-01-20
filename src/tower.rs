use bevy::{
    prelude::{
        App, BuildChildren, Commands, Component, DespawnRecursiveExt, Entity, GlobalTransform,
        Input, KeyCode, Name, Plugin, Query, Res, SpatialBundle, Transform, Vec3, With,
    },
    reflect::Reflect,
    scene::SceneBundle,
    time::{Time, Timer, TimerMode},
    utils::FloatOrd,
};
use bevy_mod_picking::Selection;

use crate::{Bullet, GameAssets, Lifetime, Target, TowerType};

#[derive(Component, Reflect, Default)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_system(tower_shooting)
            .add_system(build_tower);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    game_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, tower_type, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                let (model, bullet) = tower_type.get_bullet(direction, &game_assets);

                commands.entity(tower_ent).with_children(|commands| {
                    commands
                        .spawn(SceneBundle {
                            scene: model,
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..Default::default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1000.5, TimerMode::Once),
                        })
                        .insert(bullet)
                        .insert(Name::new("Bullet"));
                });
            }
        }
    }
}

pub fn spawn_tower(
    commands: &mut Commands,
    game_assets: &GameAssets,
    position: Vec3,
    tower_type: TowerType,
) -> Entity {
    let (tower_scene, tower) = tower_type.get_tower(game_assets);

    commands
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            position,
        )))
        .insert(Name::new(format!("{:?}_Tower", tower_type)))
        .insert(tower_type)
        .insert(tower)
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: tower_scene,
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..Default::default()
            });
        })
        .id()
}

pub fn build_tower(
    mut commands: Commands,
    selection: Query<(Entity, &Selection, &Transform)>,
    keyboard: Res<Input<KeyCode>>,
    game_assets: Res<GameAssets>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for (entity, selection, transform) in &selection {
            if selection.selected() {
                commands.entity(entity).despawn_recursive();
                spawn_tower(
                    &mut commands,
                    &game_assets,
                    transform.translation,
                    TowerType::Tomato,
                );
            }
        }
    }
}
