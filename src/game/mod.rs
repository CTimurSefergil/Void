// ============================================================================
// 🎮 GAME MODULE - Core Game Logic Organization
// ============================================================================
//
// This module organizes all game-related functionality into logical groups.
// It acts as the main hub for all game systems and mechanics.
//
// 📋 BEST PRACTICE: Modular game architecture
// - Separate different game systems into their own modules
// - Use plugin pattern to keep systems organized
// - Core mechanics include movement, AI, and world generation

pub mod core_mechanics; // AI, movement, and world generation systems
pub mod spawn;          // Entity spawning logic (player, enemies, etc.)

use bevy::prelude::*;

/// 🎯 GAME PLUGIN: Main Game Logic Registration
/// This plugin adds all game-related systems to the Bevy app
///
/// 📋 BEST PRACTICE: Plugin composition
/// - Group related plugins together
/// - Easy to enable/disable entire game systems
/// - Clear separation of concerns
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        spawn::plugin,          // Player and entity spawning
        core_mechanics::plugin, // Movement, AI, world generation
    ));
}
