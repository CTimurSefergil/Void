// ============================================================================
// 🛠️ DEBUG SYSTEM - Development Tools
// ============================================================================

use super::components::OInsanAI;
use crate::game::spawn::player::Player;
use bevy::prelude::*;

/// 🎯 SYSTEM 6: DEBUG SYSTEM
/// This provides testing controls for development
///
/// 📋 BEST PRACTICE: Always include debug tools
/// - Debug systems help you test different scenarios
/// - Use keyboard inputs for quick testing
/// - Remove or disable in release builds
pub fn ai_debug_system(
    mut ai_query: Query<&mut OInsanAI>,
    mut player_query: Query<&mut Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Damage AI (reduce health)
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        for mut ai in ai_query.iter_mut() {
            ai.health -= 20.0;
            ai.health = ai.health.max(0.0);
            println!(
                "🩸 AI damaged! Health: {:.0}/{:.0}",
                ai.health, ai.max_health
            );
        }
    }

    // Heal AI (restore health)
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        for mut ai in ai_query.iter_mut() {
            ai.health += 20.0;
            ai.health = ai.health.min(ai.max_health);
            println!(
                "❤️ AI healed! Health: {:.0}/{:.0}",
                ai.health, ai.max_health
            );
        }
    }

    // Toggle player weapon
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        for mut player in player_query.iter_mut() {
            player.has_weapon = !player.has_weapon;
            println!(
                "🗡️ Player weapon: {}",
                if player.has_weapon { "ON" } else { "OFF" }
            );
        }
    }

    // Print current AI state
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        for ai in ai_query.iter() {
            println!("📊 AI STATE:");
            println!(
                "   Health: {:.0}/{:.0} ({:.0}%)",
                ai.health,
                ai.max_health,
                (ai.health / ai.max_health) * 100.0
            );
            println!("   Emotion: {:?}", ai.emotional_state);
            println!("   Behavior: {:?}", ai.current_behavior);
            println!("   Last saw player: {:.1}s ago", ai.time_since_seen_player);
        }
    }
}
