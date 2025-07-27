// ============================================================================
// ❤️ HEALTH SYSTEM - AI Health Management and Life Cycle
// ============================================================================
//
// This system is responsible for managing AI health states, enforcing health
// constraints, and handling health-related events like death and recovery.
//
// 📋 RESPONSIBILITIES:
// - Validate and clamp health values within acceptable ranges
// - Handle death state and related cleanup
// - Provide health-related event logging
// - Ensure health system integrity and prevent invalid states

use super::super::components::OInsanAI;
use bevy::prelude::*;

/// 🎯 HEALTH MANAGEMENT SYSTEM
/// 
/// This handles health changes and ensures health stays within bounds.
/// It's a critical safety system that prevents invalid health states.
///
/// 📋 DESIGN PRINCIPLES:
/// - Always validate data integrity (health bounds)
/// - Handle edge cases gracefully (negative health, overflow)
/// - Provide clear logging for health state changes
/// - Separate health validation from health modification
pub fn ai_health_system(mut ai_query: Query<&mut OInsanAI>) {
    for mut ai in ai_query.iter_mut() {
        let previous_health = ai.health;
        
        // Ensure health stays within valid bounds
        ai.health = ai.health.clamp(0.0, ai.max_health);

        // Handle significant health state changes
        handle_health_state_changes(&ai, previous_health);
    }
}

/// Handle important health state transitions and events
fn handle_health_state_changes(ai: &OInsanAI, previous_health: f32) {
    // Check for death transition
    if previous_health > 0.0 && ai.health <= 0.0 {
        handle_death_event();
    }
    
    // Check for revival (if health was 0 and now is positive)
    if previous_health <= 0.0 && ai.health > 0.0 {
        handle_revival_event(ai.health);
    }
    
    // Check for critical health threshold
    let health_percent = ai.health / ai.max_health;
    if health_percent <= 0.1 && previous_health / ai.max_health > 0.1 {
        handle_critical_health_event(health_percent);
    }
}

/// Handle AI death event
fn handle_death_event() {
    println!("💀 AI has died!");
    // Future enhancements could include:
    // - Despawn the entity
    // - Play death animation
    // - Drop items or resources
    // - Trigger death-related game events
    // - Update game statistics
}

/// Handle AI revival event
fn handle_revival_event(new_health: f32) {
    println!("✨ AI has been revived! (Health: {:.1})", new_health);
    // Future enhancements could include:
    // - Play revival animation
    // - Reset AI state to default
    // - Trigger revival-related game events
}

/// Handle critical health warning
fn handle_critical_health_event(health_percent: f32) {
    println!(
        "⚠️ AI health critical! ({:.0}%)",
        health_percent * 100.0
    );
    // Future enhancements could include:
    // - Change AI appearance (visual damage)
    // - Trigger emergency behaviors
    // - Play critical health sound effects
    // - Modify movement speed or capabilities
}
