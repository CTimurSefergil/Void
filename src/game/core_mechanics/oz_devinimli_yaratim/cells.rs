// ============================================================================
// 🏗️ CELL SYSTEM - World Data Management
// ============================================================================
//
// This module manages the fundamental data structures for procedural world
// generation using a Wave Function Collapse-inspired algorithm. Each cell
// represents a portion of the world that can contain different tile types.
//
// 📋 BEST PRACTICE: Data-driven world generation
// - Separate data (cells) from visualization (meshes)
// - Use entropy-based algorithms for coherent generation
// - Spatial indexing for efficient cell management
// - Player-centered loading for infinite worlds

use bevy::{platform::collections::HashMap, prelude::*};
use std::{collections::HashSet, time::Duration};

use crate::game::{
    core_mechanics::oz_devinimli_yaratim::odyrules::{
        commons::TileType, open_space_rules::OpenSpaceRules,
    },
    spawn::player::Player,
};

// ============================================================================
// ⚙️ SECTION 1: CONFIGURATION CONSTANTS
// ============================================================================

/// How often to check for new cells to create (in milliseconds)
///
/// 📋 PERFORMANCE NOTE: Don't update every frame - expensive calculations
const UPDATE_INTERVAL_MS: u64 = 200;

/// How often to check for cells to remove (in milliseconds)
///
/// 📋 PERFORMANCE NOTE: Separate interval allows tuning cleanup frequency
const DESPAWN_INTERVAL_MS: u64 = 200;

// ============================================================================
// 📊 SECTION 2: RESOURCE DEFINITIONS (Global Settings)
// ============================================================================

/// Global settings for world generation behavior
///
/// 📋 BEST PRACTICE: Centralized configuration
/// - Easy to tweak world generation parameters
/// - Consistent values across all generation systems
/// - Resource pattern makes settings globally accessible
#[derive(Resource)]
pub struct GenerationSettings {
    /// How large each cell is in world units (9x9 area)
    pub cell_edge_length: i32,

    /// How many cells to generate around the player (17x17 grid)
    pub total_cells_on_edge: i32,

    /// Distance multiplier for when to despawn distant cells
    pub spawn_distance: f32,
}

impl Default for GenerationSettings {
    /// 🎯 DEFAULT SETTINGS: Balanced Performance Values
    ///
    /// 📋 BEST PRACTICE: Provide reasonable defaults
    /// - 9x9 cells provide good detail without being too small
    /// - 17x17 grid gives good view distance without performance issues
    /// - 0.7 spawn distance creates smooth loading/unloading
    fn default() -> Self {
        Self {
            cell_edge_length: 9,     // Medium-sized cells for good detail
            total_cells_on_edge: 17, // Good view distance without lag
            spawn_distance: 0.7,     // Smooth loading/unloading zone
        }
    }
}

/// Spatial index for fast cell lookup by grid coordinates
///
/// 📋 BEST PRACTICE: Spatial indexing for performance
/// - HashMap provides O(1) lookup by position
/// - Essential for large worlds with many cells
/// - Prevents duplicate cell creation
#[derive(Resource, Default)]
pub struct CellSpatialIndex {
    /// Maps grid coordinates (x, z) to cell entities
    pub grid: HashMap<(i32, i32), Entity>,
}

/// 🎯 PLUGIN SETUP: Cell System Registration
///
/// 📋 BEST PRACTICE: Initialize resources and systems together
/// - Resources provide global state and configuration
/// - Systems run in chain to ensure proper order (create before destroy)
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GenerationSettings>() // World generation settings
        .init_resource::<CellSpatialIndex>() // Spatial lookup table
        .add_systems(Update, (create_cells, destroy_cells).chain()); // Generation systems
}

// ============================================================================
// 🧩 SECTION 3: COMPONENT DEFINITIONS (Cell Data Structure)
// ============================================================================

/// A single cell in the world generation system
///
/// 📋 BEST PRACTICE: Wave Function Collapse-inspired data structure
/// - Each cell starts with all possibilities (high entropy)
/// - Constraints reduce possibilities (lower entropy)
/// - Collapsed cells have exactly one tile type
/// - Position tracking enables spatial relationships
#[derive(Component, Debug)]
pub struct Cell {
    /// Whether this cell has been finalized to a specific tile type
    pub is_collapsed: bool,

    /// The final tile type (Some when collapsed, None when still deciding)
    pub tile_type: Option<TileType>,

    /// Number of possible tile types (lower = more constrained)
    /// 📋 DESIGN NOTE: Entropy drives the Wave Function Collapse algorithm
    pub entropy: i32,

    /// List of tile types that are still possible for this cell
    pub valid_tiles: Vec<TileType>,

    /// Grid coordinates of this cell (for spatial relationships)
    pub position: (i32, i32),
}

impl Cell {
    /// 🎯 CELL CREATION: Initialize with all possibilities
    ///
    /// 📋 BEST PRACTICE: Start with maximum entropy
    /// - All tile types are initially possible
    /// - Constraints will reduce possibilities over time
    /// - Position enables spatial rule checking
    pub fn new(all_tiles: &[TileType], position: (i32, i32)) -> Self {
        Self {
            is_collapsed: false,
            tile_type: None,
            entropy: all_tiles.len() as i32, // Maximum entropy initially
            valid_tiles: all_tiles.to_vec(), // All tiles possible initially
            position,
        }
    }

    /// 🎯 ENTROPY UPDATE: Recalculate entropy after constraints
    ///
    /// 📋 BEST PRACTICE: Keep entropy in sync with valid_tiles
    /// - Entropy should always match valid_tiles.len()
    /// - Only update if cell isn't already collapsed
    /// - Essential for Wave Function Collapse algorithm
    pub fn update_entropy(&mut self) {
        if !self.is_collapsed {
            self.entropy = self.valid_tiles.len() as i32;
        }
    }

    /// 🎯 CONTRADICTION CHECK: Detect impossible cells
    ///
    /// 📋 BEST PRACTICE: Handle contradiction states
    /// - Empty valid_tiles means no solution possible
    /// - This can happen with conflicting constraints
    /// - Contradiction detection prevents infinite loops
    pub fn is_contradicted(&mut self) -> bool {
        self.valid_tiles.is_empty()
    }
}

/// Marker component for entities that represent tiles in the world
///
/// 📋 BEST PRACTICE: Use marker components for querying
/// - Makes it easy to find all tile entities
/// - Separates tiles from other world objects
/// - Essential for cleanup and management systems
#[derive(Component)]
pub struct Tile;

// ============================================================================
// 🌱 SECTION 4: CELL CREATION SYSTEM (Infinite World Loading)
// ============================================================================

/// 🎯 SYSTEM 1: CELL CREATION
/// Creates new cells around the player as they move through the world
///
/// 📋 BEST PRACTICE: Player-centered infinite world generation
/// - Only create cells near the player (performance)
/// - Use timer to prevent expensive every-frame calculations
/// - Check existing cells to prevent duplicates
/// - Grid-based positioning for consistent world structure
fn create_cells(
    mut commands: Commands,
    player_pos: Single<&Transform, With<Player>>,
    existing_cells: Query<&Transform, With<Cell>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
    wfc_rules: Res<OpenSpaceRules>,
    settings: Res<GenerationSettings>,
) {
    // Throttle updates for performance
    // 📋 PERFORMANCE NOTE: Cell creation is expensive, don't do it every frame
    let now = time.elapsed();
    if *last_update + Duration::from_millis(UPDATE_INTERVAL_MS) > now {
        return;
    }
    *last_update = now;

    // Convert player world position to grid coordinates
    let player_grid_x =
        (player_pos.translation.x / settings.cell_edge_length as f32).round() as i32;
    let player_grid_z =
        (player_pos.translation.z / settings.cell_edge_length as f32).round() as i32;

    // Build set of existing cell positions for fast lookup
    // 📋 PERFORMANCE NOTE: HashSet provides O(1) contains() checks
    let existing_positions: HashSet<(i32, i32)> = existing_cells
        .iter()
        .map(|transform| {
            let grid_x =
                (transform.translation.x / settings.cell_edge_length as f32).round() as i32;
            let grid_z =
                (transform.translation.z / settings.cell_edge_length as f32).round() as i32;
            (grid_x, grid_z)
        })
        .collect();

    // Create cells in a square around the player
    let half_size = settings.total_cells_on_edge / 2;

    for grid_x in (player_grid_x - half_size)..=(player_grid_x + half_size) {
        for grid_z in (player_grid_z - half_size)..=(player_grid_z + half_size) {
            // Skip if cell already exists
            if existing_positions.contains(&(grid_x, grid_z)) {
                continue;
            }

            // Convert grid coordinates back to world position
            let world_x = grid_x as f32 * settings.cell_edge_length as f32;
            let world_z = grid_z as f32 * settings.cell_edge_length as f32;
            let position = (grid_x, grid_z);

            // Create new cell with all tile possibilities
            let cell = Cell::new(&wfc_rules.all_tiles, position);

            // Spawn the cell entity
            commands.spawn((
                Name::new(format!("Cell_{}_{}", grid_x, grid_z)), // Helpful for debugging
                cell,
                Transform::from_translation(Vec3::new(world_x, 0.0, world_z)),
                Tile, // Marker component
            ));
        }
    }
}

// ============================================================================
// 🗑️ SECTION 5: CELL CLEANUP SYSTEM (Memory Management)
// ============================================================================

/// 🎯 SYSTEM 2: CELL DESTRUCTION
/// Removes cells that are too far from the player to save memory
///
/// 📋 BEST PRACTICE: Automatic cleanup for infinite worlds
/// - Remove distant cells to prevent memory leaks
/// - Use separate timer for cleanup (different frequency than creation)
/// - Update spatial index when removing cells
/// - Distance-based cleanup creates smooth loading zones
fn destroy_cells(
    mut commands: Commands,
    player_pos: Single<&Transform, With<Player>>,
    cells: Query<(Entity, &Transform), With<Cell>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
    settings: Res<GenerationSettings>,
    mut spatial_index: ResMut<CellSpatialIndex>,
) {
    // Throttle cleanup updates
    // 📋 PERFORMANCE NOTE: Cleanup can be less frequent than creation
    let now = time.elapsed();
    if *last_update + Duration::from_millis(DESPAWN_INTERVAL_MS) > now {
        return;
    }
    *last_update = now;

    // Calculate despawn distance based on settings
    // 📋 DESIGN NOTE: spawn_distance creates a buffer zone for smooth loading
    let despawn_distance = (settings.total_cells_on_edge as f32 * settings.cell_edge_length as f32)
        * settings.spawn_distance;

    // Check each cell for distance from player
    for (entity, transform) in cells.iter() {
        if player_pos.translation.distance(transform.translation) > despawn_distance {
            // Remove from spatial index
            // 📋 BEST PRACTICE: Keep spatial index in sync with entities
            spatial_index.grid.remove(&(
                transform.translation.x as i32,
                transform.translation.z as i32,
            ));

            // Despawn the entity
            commands.entity(entity).despawn();
        }
    }
}
