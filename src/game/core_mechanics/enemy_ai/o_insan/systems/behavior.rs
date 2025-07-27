// ============================================================================
// 🎯 BEHAVIOR SYSTEM - AI Decision Making
// ============================================================================
//
// This system is responsible for determining what the AI should do based on
// its emotional state, environmental conditions, and player interactions.
//
// 📋 RESPONSIBILITIES:
// - Analyze current situation (player position, distance, weapon status)
// - Make behavior decisions based on emotional state
// - Update AI memory (last seen player position, timestamps)
// - Optimize performance with timer-based updates

use super::super::components::{AIBehavior, EmotionalState, OInsanAI};
use crate::game::spawn::player::Player;
use bevy::prelude::*;

/// 🎯 BEHAVIOR DECISION SYSTEM
/// 
/// This decides WHAT the AI should do based on its emotions and situation.
/// It's the strategic brain that converts emotions into actionable decisions.
///
/// 📋 DESIGN PRINCIPLES:
/// - Use timers to prevent excessive calculations (performance optimization)
/// - Make decisions based on emotional state + environmental context
/// - Update AI memory for persistent behavior patterns
/// - Separate decision-making from action execution
pub fn ai_behavior_system(
    mut ai_query: Query<&mut OInsanAI>,
    player_query: Query<(&Transform, &Player), Without<OInsanAI>>,
    ai_transform_query: Query<&Transform, (With<OInsanAI>, Without<Player>)>,
    time: Res<Time>,
) {
    for mut ai in ai_query.iter_mut() {
        // Update timers for performance optimization
        ai.behavior_update_timer.tick(time.delta());
        ai.time_since_seen_player += time.delta_secs();

        // Only recalculate behavior periodically (performance!)
        if !ai.behavior_update_timer.just_finished() {
            continue;
        }

        // Gather situational context
        let context = gather_situational_context(
            &player_query,
            &ai_transform_query,
            &mut ai,
        );

        // Make behavior decision based on emotion and context
        let new_behavior = decide_behavior_from_emotion_and_context(
            ai.emotional_state,
            ai.current_behavior,
            &context,
        );

        // Update behavior if it changed
        if new_behavior != ai.current_behavior {
            log_behavior_change(&ai.current_behavior, &new_behavior);
            ai.current_behavior = new_behavior;
        }
    }
}

/// Contextual information for behavior decision making
#[derive(Debug)]
struct SituationalContext {
    player_position: Option<Vec3>,
    player_has_weapon: bool,
    ai_position: Vec3,
    distance_to_player: Option<f32>,
    can_see_player: bool,
}

/// Gather all relevant contextual information for behavior decisions
fn gather_situational_context(
    player_query: &Query<(&Transform, &Player), Without<OInsanAI>>,
    ai_transform_query: &Query<&Transform, (With<OInsanAI>, Without<Player>)>,
    ai: &mut OInsanAI,
) -> SituationalContext {
    // Get AI position
    let ai_position = if let Ok(ai_transform) = ai_transform_query.single() {
        ai_transform.translation
    } else {
        return SituationalContext {
            player_position: None,
            player_has_weapon: false,
            ai_position: Vec3::ZERO,
            distance_to_player: None,
            can_see_player: false,
        };
    };

    // Get player information
    let (player_position, player_has_weapon, distance_to_player, can_see_player) =
        if let Ok((player_transform, player)) = player_query.single() {
            let player_pos = player_transform.translation;
            let distance = ai_position.distance(player_pos);
            let can_see = distance <= ai.detection_range;
            
            // Update AI memory if we can see the player
            if can_see {
                ai.last_player_position = Some(player_pos);
                ai.time_since_seen_player = 0.0;
            }
            
            (Some(player_pos), player.has_weapon, Some(distance), can_see)
        } else {
            (None, false, None, false)
        };

    SituationalContext {
        player_position,
        player_has_weapon,
        ai_position,
        distance_to_player,
        can_see_player,
    }
}

/// Decide behavior based on emotional state and situational context
fn decide_behavior_from_emotion_and_context(
    emotional_state: EmotionalState,
    current_behavior: AIBehavior,
    context: &SituationalContext,
) -> AIBehavior {
    match emotional_state {
        EmotionalState::Depressed => decide_depressed_behavior(context),
        EmotionalState::Angry => decide_angry_behavior(current_behavior, context),
        EmotionalState::Neutral => decide_neutral_behavior(context),
    }
}

/// Behavior decisions when AI is depressed (low health)
/// Priority: Survival - escape from threats
fn decide_depressed_behavior(context: &SituationalContext) -> AIBehavior {
    if context.can_see_player {
        AIBehavior::Escaping
    } else {
        AIBehavior::Wandering
    }
}

/// Behavior decisions when AI is angry (high health)
/// Priority: Aggression - confront threats, but respect weapons
fn decide_angry_behavior(
    current_behavior: AIBehavior,
    context: &SituationalContext,
) -> AIBehavior {
    if context.can_see_player {
        // SPECIAL RULE: If player has weapon while we're being aggressive, become submissive
        if context.player_has_weapon
            && (current_behavior == AIBehavior::Chasing || current_behavior == AIBehavior::Begging)
        {
            AIBehavior::Begging
        } else {
            AIBehavior::Chasing
        }
    } else {
        AIBehavior::Wandering
    }
}

/// Behavior decisions when AI is neutral
/// Priority: Peaceful coexistence
fn decide_neutral_behavior(_context: &SituationalContext) -> AIBehavior {
    AIBehavior::Begging
}

/// Log behavior changes for debugging and game feel
fn log_behavior_change(old_behavior: &AIBehavior, new_behavior: &AIBehavior) {
    println!("🎯 AI behavior: {:?} → {:?}", old_behavior, new_behavior);
}
