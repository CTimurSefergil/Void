use bevy::prelude::*;

pub const _GROUND: [f32; 3] = [4.8, 0.1, 4.8];
pub const WALL: [f32; 3] = [4.8, 5.0, 4.8];
pub const _CORNER: [f32; 3] = [4.8, 5.0, 4.8];
pub const _CHEST: [f32; 3] = [1.5, 0.8, 1.0];

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tile_resources);
}

#[derive(Resource)]
pub struct TileModels {
    pub ground: Handle<Scene>,
    #[allow(dead_code)]
    pub wall: Handle<Scene>,
    pub corner: Handle<Scene>,
    pub chest: Handle<Scene>,
}

fn setup_tile_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_models = TileModels {
        ground: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/pallet.glb")),
        wall: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/truck-green.glb")),
        corner: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wall-a.glb")),
        chest: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/truck-green-cargo.glb")),
    };

    commands.insert_resource(tile_models);
}
