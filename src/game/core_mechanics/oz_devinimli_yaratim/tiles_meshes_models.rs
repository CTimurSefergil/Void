// ============================================================================
// 🎨 TILE VISUAL SYSTEM - 3D Model Rendering for Tiles
// ============================================================================
//
// This module handles the visual representation of generated tiles by
// converting abstract tile types into 3D models and scenes. It manages
// asset loading, model assignment, and visual updates when tiles change.
//
// 📋 BEST PRACTICE: Separation of data and visuals
// - Tile generation logic is separate from rendering
// - Models are loaded once and reused for performance
// - Visual updates only trigger when tile data changes

use bevy::prelude::*;

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    cells::{Cell, GenerationSettings},
    odyrules::commons::TileType,
};

// ============================================================================
// 📐 SECTION 1: MODEL CONSTANTS (Size Definitions)
// ============================================================================

/// Size constants for different model types
/// 
/// 📋 DESIGN NOTE: Currently unused but preserved for future scaling needs
/// - Could be used for dynamic model scaling
/// - Helpful for collision detection systems
/// - Maintains compatibility with legacy code
pub const _GROUND: [f32; 3] = [4.8, 0.1, 4.8]; // Ground tile dimensions
pub const _CORNER: [f32; 3] = [4.8, 5.0, 4.8]; // Corner structure dimensions
pub const _CHEST: [f32; 3] = [1.5, 0.8, 1.0];  // Chest object dimensions

/// 🎯 PLUGIN SETUP: Visual System Registration
/// 
/// 📋 BEST PRACTICE: Separate setup from runtime systems
/// - Startup system loads all assets once
/// - Update system only runs when tile data changes
/// - Efficient resource management
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_tile_resources)  // Load 3D models once
        .add_systems(Update, update_tile_visuals);   // Update visuals when tiles change
}

// ============================================================================
// 📦 SECTION 2: RESOURCE DEFINITIONS (Asset Management)
// ============================================================================

/// Resource containing handles to all tile 3D models
/// 
/// 📋 BEST PRACTICE: Centralized asset management
/// - All models loaded once at startup
/// - Handles are lightweight references to GPU resources
/// - Easy to add new tile types by extending this struct
#[derive(Resource)]
pub struct TileModels {
    pub ground: Handle<Scene>,           // Basic ground/road tile
    pub tree: Handle<Scene>,             // Natural tree decoration
    pub chest: Handle<Scene>,            // Interactive object (currently rock)
    pub fountain_center: Handle<Scene>,  // Central fountain piece
    pub fountain_corner: Handle<Scene>,  // Corner fountain pieces
    pub fountain_edge: Handle<Scene>,    // Edge fountain pieces
}

// ============================================================================
// 🏗️ SECTION 3: ASSET LOADING (Startup System)
// ============================================================================

/// 🎯 STARTUP SYSTEM: 3D Model Loading
/// Loads all tile models from GLTF files at game startup
/// 
/// 📋 BEST PRACTICE: Front-load assets for performance
/// - Load all models once to avoid runtime loading delays
/// - Use GLTF format for optimized 3D assets
/// - Asset server handles memory management automatically
fn setup_tile_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_models = TileModels {
        // Load scene index 0 from each GLTF file
        ground: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/road.glb")),
        tree: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/tree.glb")),
        chest: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/rockWide.glb")), // Rock as chest placeholder
        fountain_center: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/fountainCenter.glb")),
        fountain_corner: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/fountainCorner.glb")),
        fountain_edge: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("models/fountainEdge.glb")),
    };

    // Make models available as a global resource
    commands.insert_resource(tile_models);
}

// ============================================================================
// 🎭 SECTION 4: VISUAL UPDATE SYSTEM (Runtime Rendering)
// ============================================================================

/// 🎯 UPDATE SYSTEM: Tile Visual Assignment
/// Updates the visual representation when cells get assigned tile types
/// 
/// 📋 BEST PRACTICE: Change detection for performance
/// - Only processes cells that have actually changed
/// - Prevents unnecessary visual updates every frame
/// - Efficient handling of large numbers of tiles
fn update_tile_visuals(
    mut commands: Commands,
    changed_cells: Query<(Entity, &Cell, &Transform), Changed<Cell>>, // Only changed cells
    tile_models: Res<TileModels>,
    settings: Res<GenerationSettings>,
) {
    // Process each cell that has changed
    for (entity, cell, transform) in changed_cells.iter() {
        // Only update visuals for collapsed cells (cells with assigned tile types)
        if let Some(tile_type) = cell.tile_type {
            match tile_type {
                // ============================================================
                // 🌱 BASIC TILES: Ground, Trees, and Objects
                // ============================================================
                
                TileType::Ground => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x, // X position from cell
                        0.0,                          // Ground level
                        0.0 + transform.translation.z, // Z position from cell
                    ))
                    .with_scale(Vec3 {
                        x: settings.cell_edge_length as f32, // Scale to cell size
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

                // ============================================================
                // ⛲ FOUNTAIN SYSTEM: Multi-Part Structure
                // ============================================================
                
                // 📋 DESIGN NOTE: Fountain tiles form connected structures
                // - FountainCenter: Central piece (no rotation needed)
                // - FountainCorner1-4: Corner pieces with different rotations
                // - FountainEdge1-4: Edge pieces with different rotations
                // - Rotation values create proper tile alignment

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
