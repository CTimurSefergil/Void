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
    pub fountain_center: Handle<Scene>,
    pub fountain_corner: Handle<Scene>,
    pub fountain_edge: Handle<Scene>,
}

fn setup_tile_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_models = TileModels {
        ground: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/road.glb")),
        tree: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/tree.glb")),
        chest: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/rockWide.glb")),
        fountain_center: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/fountainCenter.glb")),
        fountain_corner: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/fountainCorner.glb")),
        fountain_edge: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/fountainEdge.glb")),
    };

    commands.insert_resource(tile_models);
}
