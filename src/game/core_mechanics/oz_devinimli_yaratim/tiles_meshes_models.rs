use bevy::prelude::*;

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    cells::{Cell, GenerationSettings},
    odyrules::commons::TileType,
};

pub const _GROUND: [f32; 3] = [4.8, 0.1, 4.8];
pub const _CORNER: [f32; 3] = [4.8, 5.0, 4.8];
pub const _CHEST: [f32; 3] = [1.5, 0.8, 1.0]; 

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tile_resources) 
        .add_systems(Update, update_tile_visuals);
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

fn update_tile_visuals(
    mut commands: Commands,
    changed_cells: Query<(Entity, &Cell, &Transform), Changed<Cell>>, 
    tile_models: Res<TileModels>,
    settings: Res<GenerationSettings>,
) {
    for (entity, cell, transform) in changed_cells.iter() {
        if let Some(tile_type) = cell.tile_type {
            match tile_type {
                TileType::Ground => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x, 
                        0.0,                           
                        0.0 + transform.translation.z, 
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32, 
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    });
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.ground.clone()), transform));
                }

                TileType::Tree => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    });
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.tree.clone()), transform));
                }

                TileType::Chest => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    });
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.chest.clone()), transform));
                }

                TileType::FountainCenter => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    });
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_center.clone()), transform));
                }

                TileType::FountainCorner1 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    })
                    .with_rotation(Quat::from_rotation_y(0.5 * std::f32::consts::PI)); // 90° rotation
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_corner.clone()), transform));
                }

                TileType::FountainCorner2 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    })
                    .with_rotation(Quat::from_rotation_y(1.0 * std::f32::consts::PI)); // 180° rotation
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_corner.clone()), transform));
                }

                TileType::FountainCorner3 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    }); // No rotation (0°)
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_corner.clone()), transform));
                }

                TileType::FountainCorner4 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    })
                    .with_rotation(Quat::from_rotation_y(1.5 * std::f32::consts::PI)); // 270° rotation
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_corner.clone()), transform));
                }

                TileType::FountainEdge1 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    }); // No rotation (0°)
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_edge.clone()), transform));
                }

                TileType::FountainEdge2 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    })
                    .with_rotation(Quat::from_rotation_y(0.5 * std::f32::consts::PI)); // 90° rotation
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_edge.clone()), transform));
                }

                TileType::FountainEdge3 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    })
                    .with_rotation(Quat::from_rotation_y(1.5 * std::f32::consts::PI)); // 270° rotation
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_edge.clone()), transform));
                }

                TileType::FountainEdge4 => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        0.0,
                        0.0 + transform.translation.z,
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32,
                        y: settings.cell_edge_length as f32,
                        z: settings.cell_edge_length as f32,
                    })
                    .with_rotation(Quat::from_rotation_y(1.0 * std::f32::consts::PI)); // 180° rotation
                    commands
                        .entity(entity)
                        .insert((SceneRoot(tile_models.fountain_edge.clone()), transform));
                }
            };
        }
    }
}
