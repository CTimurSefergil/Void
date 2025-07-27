// ============================================================================
// 🏭 SPAWN MODULE - Entity Creation System
// ============================================================================
//
// This module handles the creation and initialization of all game entities.
// Currently focuses on player spawning, but can be extended for enemies,
// items, and other game objects.
//
// 📋 BEST PRACTICE: Centralized entity creation
// - All entity spawning logic in one place
// - Consistent component setup
// - Easy to extend for new entity types

pub mod player; // Player entity creation and setup

use bevy::prelude::*;

/// 🎯 SPAWN PLUGIN: Entity Creation System Registration
/// Registers all entity spawning systems with the game
///
/// 📋 BEST PRACTICE: Organize spawning by entity type
/// - Each entity type has its own module
/// - Plugin pattern makes it easy to add new entity types
/// - Clear separation between different entity creation logic
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(player::plugin); // Player spawning system
}
