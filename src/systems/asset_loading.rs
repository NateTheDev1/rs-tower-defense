use bevy::{
    prelude::{AssetServer, Commands, Handle, Res, Resource},
    scene::Scene,
};

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    });
}

#[derive(Resource)]
pub struct GameAssets {
    pub bullet_scene: Handle<Scene>,
}
