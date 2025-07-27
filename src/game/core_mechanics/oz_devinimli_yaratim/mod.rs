// ============================================================================
// 🌍 OZ DEVİNİMLİ YARATIM - Procedural World Generation System
// ============================================================================
//
// "Öz Devinimli Yaratım" (Self-Dynamic Creation) - The procedural world
// generation system for Void. This module handles all aspects of creating
// dynamic, rule-based worlds and environments.
//
// 📋 BEST PRACTICE: Procedural generation architecture
// - Separate cells (data) from meshes (visual representation)
// - Rule-based generation for consistent but varied worlds
// - Core systems handle the generation logic
// - Modular design allows for different generation types

use bevy::prelude::*;

pub mod cells;              // World data structures and cell management
pub mod odycore;           // Core generation algorithms and logic
pub mod odyrules;          // Rule systems for different world types
pub mod tiles_meshes_models; // Visual representation and mesh generation

/// 🎯 PROCEDURAL GENERATION PLUGIN: World Creation System
/// Registers all world generation systems with the game
///
/// 📋 BEST PRACTICE: System initialization order
/// - cells: Data structures must be available first
/// - tiles_meshes_models: Visual systems need data structures
/// - odycore: Core logic can reference both data and visuals
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        cells::plugin,              // World data management
        tiles_meshes_models::plugin, // Visual mesh generation
        odycore::plugin,            // Core generation algorithms
    ));
}
