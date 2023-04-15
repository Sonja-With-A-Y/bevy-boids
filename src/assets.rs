use crate::*;

#[derive(Resource)]
pub struct GameAssets {
    pub boid_scene: Handle<Scene>,
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        boid_scene: assets.load("duck.glb#Scene0"),
    });
}
