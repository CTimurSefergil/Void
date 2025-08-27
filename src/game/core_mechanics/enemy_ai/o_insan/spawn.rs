use super::components::OInsanAI;
use bevy::prelude::*;

pub fn spawn_o_insan(
    commands: &mut Commands,
    position: Vec3,
    health: f32,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) -> Entity {
    let _ball_mesh = mesh_assets.add(Cuboid::default());
    let color = Color::srgb(0.8, 0.5, 0.6);
    let _ball_material = material_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });
    commands
        .spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/monster.glb"))),
            OInsanAI {
                health,
                max_health: health,
                ..Default::default()
            },
            Transform::from_translation(position),
            Name::new("TheHuman"),
        ))
        .id()
}
