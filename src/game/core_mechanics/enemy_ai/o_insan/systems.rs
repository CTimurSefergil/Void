// ============================================================================
// 🧠 SECTION 2: CORE AI LOGIC (The Decision-Making Brain)
// ============================================================================

use super::components::{AIBehavior, EmotionalState, OInsanAI};
use crate::game::spawn::player::Player;
use bevy::prelude::*;
use rand::{prelude::*, rng};

/// 🎯 SYSTEM 1: EMOTION CALCULATION
/// This is the HEART of the AI - it decides how the AI feels based on health
///
/// 📋 BEST PRACTICE: Separate emotion from behavior
/// - Emotions are INTERNAL feelings
/// - Behaviors are EXTERNAL actions
/// - This separation makes the AI more flexible and easier to debug
pub fn ai_emotion_system(mut ai_query: Query<&mut OInsanAI>, _time: Res<Time>) {
    for mut ai in ai_query.iter_mut() {
        // Calculate health percentage (0.0 to 1.0)
        let health_percent = ai.health / ai.max_health;

        // EMOTION RULES: Simple thresholds
        let new_emotion = match health_percent {
            hp if hp <= 0.3 => EmotionalState::Depressed, // 0-30%: Depressed
            hp if hp >= 0.7 => EmotionalState::Angry,     // 70-100%: Angry
            _ => EmotionalState::Neutral,                 // 30-70%: Neutral
        };

        // Only change if emotion actually changed (avoid spam)
        if new_emotion != ai.emotional_state {
            ai.emotional_state = new_emotion;
            println!(
                "🧠 AI emotion changed to: {:?} (Health: {:.0}%)",
                new_emotion,
                health_percent * 100.0
            );
        }
    }
}

/// 🎯 SYSTEM 2: BEHAVIOR DECISION
/// This decides WHAT the AI should do based on its emotions and situation
///
/// 📋 BEST PRACTICE: Use timers to prevent excessive calculations
/// - Don't recalculate behavior every frame (expensive!)
/// - Use timers to update only when needed
/// - This improves performance and makes behavior more stable
pub fn ai_behavior_system(
    mut ai_query: Query<&mut OInsanAI>,
    player_query: Query<(&Transform, &Player), Without<OInsanAI>>,
    ai_transform_query: Query<&Transform, (With<OInsanAI>, Without<Player>)>,
    time: Res<Time>,
) {
    for mut ai in ai_query.iter_mut() {
        // Update timers
        ai.behavior_update_timer.tick(time.delta());
        ai.time_since_seen_player += time.delta_secs();

        // Only recalculate behavior periodically (performance!)
        if !ai.behavior_update_timer.just_finished() {
            continue;
        }

        // Find the player and AI positions
        let (player_pos, player_has_weapon) =
            if let Ok((player_transform, player)) = player_query.single() {
                (Some(player_transform.translation), player.has_weapon)
            } else {
                (None, false)
            };

        let ai_pos = if let Ok(ai_transform) = ai_transform_query.single() {
            ai_transform.translation
        } else {
            continue; // No AI transform found
        };

        // BEHAVIOR DECISION LOGIC
        let new_behavior = match ai.emotional_state {
            EmotionalState::Depressed => {
                // When depressed, always try to escape if we can see the player
                if let Some(player_position) = player_pos {
                    let distance = ai_pos.distance(player_position);
                    if distance <= ai.detection_range {
                        ai.last_player_position = Some(player_position);
                        ai.time_since_seen_player = 0.0;
                        AIBehavior::Escaping
                    } else {
                        AIBehavior::Wandering
                    }
                } else {
                    AIBehavior::Wandering
                }
            }

            EmotionalState::Angry => {
                // When angry, chase the player if we can see them
                if let Some(player_position) = player_pos {
                    let distance = ai_pos.distance(player_position);
                    if distance <= ai.detection_range {
                        ai.last_player_position = Some(player_position);
                        ai.time_since_seen_player = 0.0;

                        // SPECIAL RULE: If player has weapon while we're chasing, become sorrowful
                        if player_has_weapon && ai.current_behavior == AIBehavior::Chasing {
                            ai.emotional_state = EmotionalState::Sorrow;
                            println!("😢 AI sees weapon while chasing - becoming sorrowful!");
                            AIBehavior::Begging
                        } else {
                            AIBehavior::Chasing
                        }
                    } else {
                        AIBehavior::Wandering
                    }
                } else {
                    AIBehavior::Wandering
                }
            }

            EmotionalState::Sorrow => {
                // When in sorrow, just stand and beg
                AIBehavior::Begging
            }

            EmotionalState::Neutral => {
                // When neutral, just wander around
                AIBehavior::Wandering
            }
        };

        // Update behavior if it changed
        if new_behavior != ai.current_behavior {
            ai.current_behavior = new_behavior;
            println!("🎯 AI behavior changed to: {:?}", new_behavior);
        }
    }
}

// ============================================================================
// 🏃 SECTION 3: MOVEMENT SYSTEMS (Making the AI Move)
// ============================================================================

/// 🎯 SYSTEM 3: MOVEMENT EXECUTION
/// This actually moves the AI based on its current behavior
///
/// 📋 BEST PRACTICE: Separate decision-making from execution
/// - Decision systems determine WHAT to do
/// - Execution systems determine HOW to do it
/// - This makes the code modular and easy to modify
pub fn ai_movement_system(
    mut ai_query: Query<(&mut Transform, &OInsanAI), Without<Player>>,
    player_query: Query<&Transform, (With<Player>, Without<OInsanAI>)>,
    time: Res<Time>,
) {
    for (mut ai_transform, ai) in ai_query.iter_mut() {
        let delta_time = time.delta_secs();
        let movement_distance = ai.movement_speed * delta_time;

        match ai.current_behavior {
            AIBehavior::Wandering => {
                // Random wandering - simple but effective
                let mut rng = rng();
                let random_direction = Vec3::new(
                    rng.random_range(-1.0..1.0),
                    0.0, // Keep on ground
                    rng.random_range(-1.0..1.0),
                )
                .normalize_or_zero();

                ai_transform.translation += random_direction * movement_distance * 0.5; // Slow wandering
            }

            AIBehavior::Chasing => {
                // Chase the player if we can find them
                if let Ok(player_transform) = player_query.single() {
                    let direction = (player_transform.translation - ai_transform.translation)
                        .normalize_or_zero();
                    ai_transform.translation += direction * movement_distance;

                    // Face the player (optional - makes it look more natural)
                    ai_transform.look_at(player_transform.translation, Vec3::Y);
                }
            }

            AIBehavior::Escaping => {
                // Run away from the player
                if let Some(last_player_pos) = ai.last_player_position {
                    let direction =
                        (ai_transform.translation - last_player_pos).normalize_or_zero();
                    ai_transform.translation += direction * movement_distance * 1.5; // Fast escape
                } else if let Ok(player_transform) = player_query.single() {
                    // If we can see the player, run directly away
                    let direction = (ai_transform.translation - player_transform.translation)
                        .normalize_or_zero();
                    ai_transform.translation += direction * movement_distance * 1.5;
                }
            }

            AIBehavior::Begging => {
                // Stand still and face the player
                if let Ok(player_transform) = player_query.single() {
                    ai_transform.look_at(player_transform.translation, Vec3::Y);
                }
                // No movement when begging
            }
        }
    }
}

// ============================================================================
// 🗣️ SECTION 4: COMMUNICATION SYSTEM (Making the AI Talk)
// ============================================================================

/// 🎯 SYSTEM 4: AI SPEECH
/// This makes the AI say things based on its current state
///
/// 📋 BEST PRACTICE: Use timers to prevent spam
/// - Without timers, AI would talk every frame (60+ times per second!)
/// - Timers make speech feel natural and readable
pub fn ai_speech_system(mut ai_query: Query<&mut OInsanAI>, time: Res<Time>) {
    for mut ai in ai_query.iter_mut() {
        ai.speech_timer.tick(time.delta());

        // Only speak when timer finishes
        if ai.speech_timer.just_finished() {
            let dialogue = match (ai.emotional_state, ai.current_behavior) {
                // Depressed and escaping
                (EmotionalState::Depressed, AIBehavior::Escaping) => vec![
                    "Leave me alone...",
                    "I can't take this anymore...",
                    "Why won't you just go away?",
                ],

                // Angry and chasing
                (EmotionalState::Angry, AIBehavior::Chasing) => vec![
                    "Come here!",
                    "You can't escape me!",
                    "I'm going to get you!",
                ],

                // Sorrowful and begging - THE SPECIAL CASE
                (EmotionalState::Sorrow, AIBehavior::Begging) => vec![
                    "I love you!", // The specific line you wanted
                    "Please don't hurt me!",
                    "I'm sorry for chasing you!",
                ],

                // Neutral wandering
                (EmotionalState::Neutral, AIBehavior::Wandering) => {
                    vec!["Where am I?", "Just walking around...", "Nothing to do..."]
                }

                // Fallback for any other combinations
                _ => vec!["..."],
            };

            // Randomly pick a line and say it (80% chance to avoid too much chatter)
            let mut rng = rng();
            if rng.random_bool(0.8) {
                let line = dialogue[rng.random_range(0..dialogue.len())];
                println!("🤖 AI says: '{}'", line);
            }
        }
    }
}

// ============================================================================
// 🔧 SECTION 5: UTILITY SYSTEMS (Helper Functions)
// ============================================================================

/// 🎯 SYSTEM 5: HEALTH MANAGEMENT
/// This handles health changes and ensures health stays within bounds
///
/// 📋 BEST PRACTICE: Always validate data
/// - Clamp health between 0 and max_health
/// - Handle edge cases (negative health, overflow, etc.)
pub fn ai_health_system(mut ai_query: Query<&mut OInsanAI>) {
    for mut ai in ai_query.iter_mut() {
        // Ensure health stays within valid bounds
        ai.health = ai.health.clamp(0.0, ai.max_health);

        // Optional: Handle death
        if ai.health <= 0.0 {
            println!("💀 AI has died!");
            // You could despawn the entity here, play death animation, etc.
        }
    }
}
