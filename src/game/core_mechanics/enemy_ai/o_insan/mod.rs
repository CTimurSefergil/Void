// ============================================================================
// 🤖 O_INSAN AI SYSTEM - Module Declaration
// ============================================================================

use bevy::prelude::*;

use crate::game::core_mechanics::enemy_ai::o_insan::spawn::spawn_simple_ai;

pub mod components;
pub mod debug;
pub mod spawn;
pub mod systems;

// Re-export the commonly used items

/// The main AI plugin that adds all systems to the game
///
/// 📋 BEST PRACTICE: Use plugins to organize systems
/// - Groups related systems together
/// - Makes it easy to enable/disable AI
/// - Follows Bevy's architecture patterns
pub struct SimpleAIPlugin;

impl Plugin for SimpleAIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup systems (run once at startup)
            .add_systems(Startup, setup_ai_demo)
            // Main AI systems (run every frame, in order)
            .add_systems(
                Update,
                (
                    systems::ai_emotion_system,  // 1. Calculate emotions
                    systems::ai_behavior_system, // 2. Decide behavior
                    systems::ai_movement_system, // 3. Execute movement
                    systems::ai_speech_system,   // 4. Handle speech
                    systems::ai_health_system,   // 5. Manage health
                    debug::ai_debug_system,      // 6. Debug controls
                )
                    .chain(),
            ); // .chain() ensures systems run in the specified order
    }
}

/// Initial setup - creates AI and player for testing
fn setup_ai_demo(
    mut commands: Commands,
    mesh_assets: ResMut<Assets<Mesh>>,
    material_assets: ResMut<Assets<StandardMaterial>>,
) {
    // Create one AI with full health (will be angry)
    let _ai_entity = spawn_simple_ai(
        &mut commands,
        Vec3::new(0.0, 0.0, 0.0),
        100.0,
        mesh_assets,
        material_assets,
    );

    println!("🎮 SIMPLE AI DEMO STARTED!");
    println!("🤖 AI spawned at (0, 0, 0) with full health (angry)");
    println!("👤 Player spawned at (5, 0, 0)");
    println!();
    println!("🎮 DEBUG CONTROLS:");
    println!("   Press '1' to damage AI (-20 health)");
    println!("   Press '2' to heal AI (+20 health)");
    println!("   Press '3' to toggle player weapon");
    println!("   Press '4' to print AI state");
    println!();
    println!("🎯 EXPECTED BEHAVIOR:");
    println!("   High Health (70-100%): AI will be ANGRY and CHASE player");
    println!("   Medium Health (30-70%): AI will be NEUTRAL and WANDER");
    println!("   Low Health (0-30%): AI will be DEPRESSED and try to ESCAPE");
    println!(
        "   Special: If chasing + player has weapon = AI becomes SORROWFUL and says 'I love you'"
    );
}
