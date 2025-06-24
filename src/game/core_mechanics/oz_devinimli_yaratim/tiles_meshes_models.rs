use bevy::prelude::*;

pub const _GROUND: [f32; 3] = [4.8, 0.1, 4.8];
pub const _CORNER: [f32; 3] = [4.8, 5.0, 4.8];
pub const _CHEST: [f32; 3] = [1.5, 0.8, 1.0];

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tile_resources);
}

#[derive(Resource)]
pub struct TileModels {
    pub ground: Handle<Scene>,
    pub tree: Handle<Scene>,
    pub chest: Handle<Scene>,
<<<<<<< Updated upstream
    #[allow(dead_code)]
    pub wall: Handle<Scene>,
    #[allow(dead_code)]
    pub corner: Handle<Scene>,
    #[allow(dead_code)]
    pub floor: Handle<Scene>,
    #[allow(dead_code)]
    pub door: Handle<Scene>,
    #[allow(dead_code)]
    pub window: Handle<Scene>,
    #[allow(dead_code)]
    pub stairs: Handle<Scene>,
=======
>>>>>>> Stashed changes
}

fn setup_tile_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_models = TileModels {
        ground: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/road.glb")),
        tree: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/tree.glb")),
        chest: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/rockWide.glb")),
<<<<<<< Updated upstream

        wall: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wall.glb")),
        corner: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/corner.glb")),
        floor: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/buildingGround.glb")),
        door: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/door.glb")),
        window: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/window.glb")),
        stairs: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/stairs.glb")),
=======
>>>>>>> Stashed changes
    };

    commands.insert_resource(tile_models);
}
