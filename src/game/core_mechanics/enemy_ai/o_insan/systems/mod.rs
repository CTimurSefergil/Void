// ============================================================================
// 🧠 AI SYSTEMS - Modular Architecture
// ============================================================================
//
// This module organizes AI systems by their specific responsibilities,
// following the Single Responsibility Principle and Separation of Concerns.
//
// 📋 ARCHITECTURAL BENEFITS:
// - Each system has a single, clear responsibility
// - Systems are easier to test independently
// - Code is more maintainable and scalable
// - Dependencies are explicit and minimal
// - Systems can be easily reused or replaced

pub mod emotion;    // Handles emotional state calculation
pub mod behavior;   // Handles behavior decision making
pub mod movement;   // Handles movement execution and pathfinding
pub mod speech;     // Handles AI communication and dialogue
pub mod health;     // Handles health management and death

// Re-export all systems for easy access
pub use emotion::ai_emotion_system;
pub use behavior::ai_behavior_system;
pub use movement::ai_movement_system;
pub use speech::ai_speech_system;
pub use health::ai_health_system;
