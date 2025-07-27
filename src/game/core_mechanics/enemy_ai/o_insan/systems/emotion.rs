// ============================================================================
// 🧠 EMOTION SYSTEM - AI Emotional State Management
// ============================================================================
//
// This system is responsible for calculating and updating the AI's emotional state
// based on its current health and other factors.
//
// 📋 RESPONSIBILITIES:
// - Calculate emotional state based on health percentage
// - Update emotional state when conditions change
// - Provide emotional state transitions and logging

use super::super::components::{EmotionalState, OInsanAI};
use bevy::prelude::*;

/// 🎯 EMOTION CALCULATION SYSTEM
/// 
/// This is the HEART of the AI - it decides how the AI feels based on health
/// and other contextual factors.
///
/// 📋 DESIGN PRINCIPLES:
/// - Emotions are INTERNAL feelings, not behaviors
/// - Emotions drive behavior decisions, but don't directly control actions
/// - Emotional transitions are logged for debugging and immersion
/// - System is stateless and purely reactive to current conditions
pub fn ai_emotion_system(mut ai_query: Query<&mut OInsanAI>, _time: Res<Time>) {
    for mut ai in ai_query.iter_mut() {
        let new_emotion = calculate_emotion_from_health(ai.health, ai.max_health);

        // Only change if emotion actually changed (avoid spam)
        if new_emotion != ai.emotional_state {
            log_emotion_change(&ai.emotional_state, &new_emotion, ai.health, ai.max_health);
            ai.emotional_state = new_emotion;
        }
    }
}

/// Calculate emotional state based on health percentage
/// 
/// 📋 EMOTION RULES:
/// - 0-30% Health: Depressed (survival mode, wants to escape)
/// - 70-100% Health: Angry (confident, aggressive, chases threats)
/// - 30-70% Health: Neutral (calm, default behavior)
fn calculate_emotion_from_health(health: f32, max_health: f32) -> EmotionalState {
    let health_percent = health / max_health;
    
    match health_percent {
        hp if hp <= 0.3 => EmotionalState::Depressed,
        hp if hp >= 0.7 => EmotionalState::Angry,
        _ => EmotionalState::Neutral,
    }
}

/// Log emotional state changes for debugging and immersion
fn log_emotion_change(
    old_emotion: &EmotionalState,
    new_emotion: &EmotionalState,
    current_health: f32,
    max_health: f32,
) {
    let health_percent = (current_health / max_health) * 100.0;
    println!(
        "🧠 AI emotion: {:?} → {:?} (Health: {:.0}%)",
        old_emotion, new_emotion, health_percent
    );
}
