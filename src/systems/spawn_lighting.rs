use std::f32::consts::PI;

use bevy::prelude::{
    default, Commands, DirectionalLight, DirectionalLightBundle, Name, OrthographicProjection,
    PointLight, PointLightBundle, Quat, Transform, Vec3,
};

pub fn spawn_lighting(mut commands: Commands) {
    const HALF_SIZE: f32 = 10.0;

    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         // Configure the projection to better fit the scene
    //         shadow_projection: OrthographicProjection {
    //             left: -HALF_SIZE,
    //             right: HALF_SIZE,
    //             bottom: -HALF_SIZE,
    //             top: HALF_SIZE,
    //             near: -10.0 * HALF_SIZE,
    //             far: 10.0 * HALF_SIZE,
    //             ..default()
    //         },
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     ..default()
    // });

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
