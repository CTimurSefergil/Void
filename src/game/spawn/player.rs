use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        })
        .with_scale(Vec3 {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        }),
        Player,
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/truck-green.glb"))),
    ));
}
