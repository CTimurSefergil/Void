// ============================================================================
// ⚙️ CORE MECHANICS MODULE - Game System Organization
// ============================================================================
//
// This module contains all the core gameplay mechanics that make Void work:
// - Player movement and controls
// - AI behavior and decision-making
// - Procedural world generation (oz_devinimli_yaratim)
//
// 📋 BEST PRACTICE: Logical system grouping
// - Each mechanic has its own module for clarity
// - Plugin pattern makes systems easy to enable/disable
// - Clear separation between different game mechanics

use bevy::prelude::*;

pub mod enemy_ai;            // AI behavior, emotions, and decision-making
pub mod movement;            // Player movement and camera controls  
pub mod oz_devinimli_yaratim; // Procedural world generation system

/// 🎯 CORE MECHANICS PLUGIN: Game System Registration
/// Registers all core gameplay mechanics with the Bevy app
///
/// 📋 BEST PRACTICE: System initialization order
/// - Movement should be available before AI (AI might react to player movement)
/// - World generation should be available early for spawning entities
/// - AI systems can reference player and world state
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(movement::plugin);            // Player controls and movement
    app.add_plugins(oz_devinimli_yaratim::plugin); // World generation
    app.add_plugins(enemy_ai::plugin);            // AI behavior systems
}
