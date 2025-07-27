// ============================================================================
// 🗣️ SPEECH SYSTEM - AI Communication and Dialogue
// ============================================================================
//
// This system is responsible for managing AI speech patterns, dialogue selection,
// and communication timing based on the AI's current emotional and behavioral state.
//
// 📋 RESPONSIBILITIES:
// - Generate contextual dialogue based on emotional state and behavior
// - Manage speech timing to prevent spam and create natural conversation flow
// - Provide varied dialogue options for immersion and personality
// - Handle special dialogue cases and emotional expressions

use super::super::components::{AIBehavior, EmotionalState, OInsanAI};
use bevy::prelude::*;
use rand::{prelude::*, rng};

/// 🎯 AI SPEECH SYSTEM
/// 
/// This makes the AI communicate based on its current emotional and behavioral state.
/// Speech adds personality and helps players understand the AI's internal state.
///
/// 📋 DESIGN PRINCIPLES:
/// - Use timers to prevent speech spam and create natural pacing
/// - Different emotional states have different dialogue pools
/// - Contextual dialogue based on behavior combinations
/// - Random selection with probability control for variety without overwhelming chatter
pub fn ai_speech_system(mut ai_query: Query<&mut OInsanAI>, time: Res<Time>) {
    for mut ai in ai_query.iter_mut() {
        ai.speech_timer.tick(time.delta());

        // Only speak when timer finishes (natural pacing)
        if ai.speech_timer.just_finished() {
            attempt_speech(&ai);
        }
    }
}

/// Attempt to generate speech based on current AI state
fn attempt_speech(ai: &OInsanAI) {
    let mut rng = rng();
    
    // 80% chance to speak when timer finishes (avoids overwhelming chatter)
    if !rng.random_bool(0.8) {
        return;
    }

    let dialogue_options = select_dialogue_for_state(ai.emotional_state, ai.current_behavior);
    
    if !dialogue_options.is_empty() {
        let selected_line = dialogue_options[rng.random_range(0..dialogue_options.len())];
        println!("🤖 AI says: '{}'", selected_line);
    }
}

/// Select appropriate dialogue based on emotional state and current behavior
fn select_dialogue_for_state(
    emotional_state: EmotionalState,
    current_behavior: AIBehavior,
) -> Vec<&'static str> {
    match (emotional_state, current_behavior) {
        // Depressed + Escaping: Desperate, pleading dialogue
        (EmotionalState::Depressed, AIBehavior::Escaping) => vec![
            "Leave me alone...",
            "I can't take this anymore...",
            "Why won't you just go away?",
            "Please, I just want to be left in peace...",
            "I'm too tired for this...",
        ],

        // Depressed + Wandering: Melancholic, lost dialogue
        (EmotionalState::Depressed, AIBehavior::Wandering) => vec![
            "Everything hurts...",
            "What's the point anymore?",
            "I feel so empty...",
            "Is this all there is?",
        ],

        // Angry + Chasing: Aggressive, threatening dialogue
        (EmotionalState::Angry, AIBehavior::Chasing) => vec![
            "Come here!",
            "You can't escape me!",
            "I'm going to get you!",
            "Stop running!",
            "You're mine now!",
        ],

        // Angry + Begging: Conflicted, submissive dialogue (special weapon case)
        (EmotionalState::Angry, AIBehavior::Begging) => vec![
            "I... I love you...",
            "Please don't hurt me...",
            "I don't want to fight anymore...",
            "That weapon scares me...",
        ],

        // Angry + Wandering: Frustrated, searching dialogue
        (EmotionalState::Angry, AIBehavior::Wandering) => vec![
            "Where did you go?",
            "I know you're out there...",
            "Come out and face me!",
            "Hiding won't save you...",
        ],

        // Neutral + Wandering: Calm, observational dialogue
        (EmotionalState::Neutral, AIBehavior::Wandering) => vec![
            "Where am I?",
            "Just walking around...",
            "Nothing to do...",
            "Peaceful day...",
            "Wonder what's over there...",
        ],

        // Neutral + Begging: Friendly, affectionate dialogue
        (EmotionalState::Neutral, AIBehavior::Begging) => vec![
            "I love you...",
            "You seem nice...",
            "Want to be friends?",
            "I like your company...",
            "Stay with me...",
        ],

        // Fallback for any unhandled combinations
        _ => vec!["..."],
    }
}
