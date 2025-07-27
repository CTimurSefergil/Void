// ============================================================================
// 🤖 ENEMY AI MODULE - Artificial Intelligence System
// ============================================================================
//
// This module contains all AI-related systems for enemies and NPCs in Void.
// Currently includes the "o_insan" (human) AI system with emotional states
// and dynamic behavior patterns.
//
// 📋 BEST PRACTICE: AI system organization
// - Each AI type has its own module (o_insan for humans)
// - Modular design allows for different AI behaviors
// - Easy to add new AI types (animals, robots, etc.)

use bevy::prelude::*;

pub mod o_insan; // Human AI with emotional states and behaviors

/// 🎯 AI PLUGIN: Enemy AI System Registration
/// Registers all AI systems with the game
///
/// 📋 BEST PRACTICE: Centralized AI management
/// - All AI types registered in one place
/// - Easy to enable/disable different AI systems
/// - Clear overview of all active AI behaviors
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(o_insan::SimpleAIPlugin); // Human AI with emotions
}
