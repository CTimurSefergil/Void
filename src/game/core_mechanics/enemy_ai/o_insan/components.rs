// ============================================================================
// 📊 SECTION 1: CORE DATA STRUCTURES (The AI's Foundation)
// ============================================================================

use bevy::prelude::*;

/// The three basic emotions our AI can feel
/// Keep it simple - only what we actually need!
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmotionalState {
    Depressed, // Low health - wants to escape
    Angry,     // High health - aggressive, chases player
    Sorrow,    // Sees weapon while chasing - begs for mercy
    Neutral,   // Default state - just wanders
}

/// Current behavior the AI is executing
/// This is what the AI is actually DOING right now
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIBehavior {
    Wandering, // Moving around randomly
    Chasing,   // Following the player
    Escaping,  // Running away from player
    Begging,   // Standing still, saying "I love you"
}

/// The main AI brain - this controls everything about the AI
#[derive(Component)]
pub struct OInsanAI {
    // CORE STATE
    pub health: f32,                     // Current health (0.0 to 100.0)
    pub max_health: f32,                 // Maximum health
    pub emotional_state: EmotionalState, // How the AI feels
    pub current_behavior: AIBehavior,    // What the AI is doing

    // TIMERS (prevent spam and control timing)
    pub behavior_update_timer: Timer, // How often to recalculate behavior
    pub speech_timer: Timer,          // How often AI can speak

    // MEMORY (what the AI remembers)
    pub last_player_position: Option<Vec3>, // Where did we last see the player?
    pub time_since_seen_player: f32,        // How long since we saw the player?

    // CONFIGURATION (tweakable values)
    pub movement_speed: f32,  // How fast the AI moves
    pub detection_range: f32, // How far the AI can "see" the player
}

impl Default for OInsanAI {
    fn default() -> Self {
        Self {
            // Start with good health and neutral mood
            health: 100.0,
            max_health: 100.0,
            emotional_state: EmotionalState::Neutral,
            current_behavior: AIBehavior::Wandering,

            // Update behavior every 0.5 seconds (don't spam calculations)
            behavior_update_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            speech_timer: Timer::from_seconds(2.0, TimerMode::Repeating),

            // No memory yet
            last_player_position: None,
            time_since_seen_player: 0.0,

            // Reasonable default values
            movement_speed: 10.0,
            detection_range: 40.0,
        }
    }
}
