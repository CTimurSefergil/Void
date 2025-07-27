// ============================================================================
// 🧠 ODYCORE - Core Generation Algorithms
// ============================================================================
//
// This module contains the core algorithms for procedural world generation
// using Wave Function Collapse (WFC) principles. It handles the actual
// generation logic, constraint propagation, and cell collapse operations.
//
// 📋 BEST PRACTICE: Centralized generation algorithms
// - Separate algorithm logic from data structures
// - Use queue-based constraint propagation for consistency
// - Chain systems in proper order for reliable generation

use bevy::{
    app::{App, Startup, Update},
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Res},
    },
};

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    cells::CellSpatialIndex,
    odycore::open_space::{
        OpenSpacePropagationQueue, collapse_lowest_entropy_open_space_cell, initialize_new_cells,
        propagate_open_space_constraints, update_spatial_index,
    },
    odyrules::open_space_rules::OpenSpaceRules,
};

pub mod open_space; // Open space generation algorithms

/// 🎯 CORE GENERATION PLUGIN: Wave Function Collapse System
/// Registers all core generation systems with proper ordering
///
/// 📋 BEST PRACTICE: Wave Function Collapse system ordering
/// - update_spatial_index: Keep track of cell positions first
/// - initialize_new_cells: Set up new cells with full entropy
/// - propagate_open_space_constraints: Apply rules to reduce entropy
/// - collapse_lowest_entropy_cell: Only when propagation is complete
/// - Chain ensures systems run in the correct sequence
pub fn plugin(app: &mut App) {
    app.init_resource::<OpenSpaceRules>() // Rule definitions for generation
        .init_resource::<OpenSpacePropagationQueue>() // Queue for constraint propagation
        .add_systems(Startup, setup_wfc_rules) // Initialize all resources
        .add_systems(
            Update,
            (
                update_spatial_index,             // 1. Update cell position tracking
                initialize_new_cells,             // 2. Initialize new cells with rules
                propagate_open_space_constraints, // 3. Apply constraints to reduce entropy
                collapse_lowest_entropy_open_space_cell.run_if(propagation_queue_empty), // 4. Collapse when ready
            )
                .chain(), // 📋 CRITICAL: Chain ensures proper execution order
        );
}

/// 🎯 CONDITION CHECK: Propagation Queue Status
/// Determines if constraint propagation is complete
///
/// 📋 BEST PRACTICE: Use run conditions for system control
/// - Only collapse cells when all constraints have been applied
/// - Prevents premature collapse that could create inconsistencies
/// - Queue-based approach ensures all rules are processed
fn propagation_queue_empty(queue: Res<OpenSpacePropagationQueue>) -> bool {
    queue.queue.is_empty() // True when no more constraints to propagate
}

/// 🎯 STARTUP SYSTEM: WFC Resource Initialization
/// Sets up all necessary resources for Wave Function Collapse generation
///
/// 📋 BEST PRACTICE: Initialize all resources at startup
/// - OpenSpaceRules: Defines tile compatibility rules
/// - CellSpatialIndex: Fast spatial lookup for cells
/// - OpenSpacePropagationQueue: Manages constraint propagation order
fn setup_wfc_rules(mut commands: Commands) {
    commands.insert_resource(OpenSpaceRules::default()); // Tile placement rules
    commands.insert_resource(CellSpatialIndex::default()); // Spatial indexing
    commands.insert_resource(OpenSpacePropagationQueue::default()); // Constraint queue
}
