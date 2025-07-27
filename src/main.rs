// ============================================================================
// 🎮 VOID GAME - MAIN ENTRY POINT
// ============================================================================
//
// This is the heart of the Void game - where everything begins!
//
// 📋 BEST PRACTICE: Keep main.rs minimal and organized
// - Import only necessary modules
// - Use plugin system to organize features
// - Keep initialization logic simple and clear

use bevy::prelude::*;

mod camera;
mod game;

/// 🎯 MAIN FUNCTION: Game Initialization
/// This function starts the entire Void game by setting up Bevy and our plugins
///
/// 📋 BEST PRACTICE: Plugin-based architecture
/// - DefaultPlugins provides core Bevy functionality (rendering, input, audio, etc.)
/// - camera::plugin handles all camera-related systems
/// - game::plugin contains all game logic (AI, movement, world generation)
fn main() {
    App::new()
        // Core Bevy systems (rendering, input, windowing, etc.)
        .add_plugins(DefaultPlugins)
        // Camera control and following system
        .add_plugins(camera::plugin)
        // All game logic (player, AI, world generation)
        .add_plugins(game::plugin)
        // Start the game loop!
        .run();
}
