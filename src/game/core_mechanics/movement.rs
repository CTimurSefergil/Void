// ============================================================================
// 🏃 PLAYER MOVEMENT SYSTEM - Controls and Input Handling
// ============================================================================
//
// This module handles all player movement and interaction controls:
// - WASD/Arrow key movement in 3D space
// - Mouse look for camera/player rotation
// - Window focus handling for smooth gameplay
// - Cursor grab/release for immersive experience
//
// 📋 BEST PRACTICE: Input system organization
// - Separate movement from looking for cleaner code
// - Handle window focus to prevent unwanted input
// - Use events for state changes (grab/ungrab cursor)

use bevy::{
    input::{common_conditions::input_just_released, mouse::AccumulatedMouseMotion},
    prelude::*,
    window::PrimaryWindow,
};

use crate::game::spawn::player::Player;

/// Movement speed constant - how fast the player moves in units per second
/// 
/// 📋 BEST PRACTICE: Use constants for tweakable values
/// - Easy to adjust gameplay feel
/// - Centralized configuration
/// - Clear what the value represents
const MOVEMENT_SPEED: f32 = 23.0;

/// 🎯 PLUGIN SETUP: Movement System Registration
/// Registers all movement and input systems with proper ordering
///
/// 📋 BEST PRACTICE: System ordering and conditions
/// - player_movement runs after player_look (rotation affects movement direction)
/// - toggle_grab only runs when Escape is released (prevents spam)
/// - Observer pattern for event handling (apply_grab)
pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_movement.after(player_look), // Movement uses rotation from looking
            player_look,                        // Handle mouse input for rotation
            focus_event,                        // Handle window focus changes
            toggle_grab.run_if(input_just_released(KeyCode::Escape)), // Escape to toggle cursor
        ),
    )
    .add_observer(apply_grab); // Event observer for cursor grab changes
}

// ============================================================================
// 📡 SECTION 1: EVENT SYSTEM (Cursor Grab Control)
// ============================================================================

/// Event for controlling cursor grab state
/// 
/// 📋 BEST PRACTICE: Use events for state changes
/// - Decouples input detection from state application
/// - Multiple systems can react to the same event
/// - Clean separation of concerns
#[derive(Event, Deref)]
struct GrabEvent(bool); // true = grab cursor, false = release cursor

// ============================================================================
// 🎮 SECTION 2: PLAYER MOVEMENT (WASD Controls)
// ============================================================================

/// 🎯 SYSTEM 1: PLAYER MOVEMENT
/// Handles WASD/Arrow key input for 3D movement
///
/// 📋 BEST PRACTICE: Direction-based movement
/// - Movement is relative to where player is looking
/// - Normalize movement vector to prevent faster diagonal movement
/// - Use transform.forward() and transform.right() for proper 3D movement
fn player_movement(
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Calculate movement intent from input
    let mut intent = Vec3::ZERO;
    
    // Forward/Backward movement (W/S or Arrow Up/Down)
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.z += 1.0; // Forward in local space
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.z -= 1.0; // Backward in local space
    }
    
    // Left/Right movement (A/D or Arrow Left/Right)
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0; // Left in local space
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0; // Right in local space
    }

    // Convert local movement intent to world space movement
    // 📋 CRITICAL: These 4 lines are essential for proper directional movement
    // Without them, movement would be global rather than relative to look direction
    let forward = player.forward().as_vec3() * intent.z; // Forward/backward in look direction
    let right = player.right().as_vec3() * intent.x;     // Left/right relative to look direction
    let mut to_move = forward + right;                   // Combine movement vectors
    to_move.y = 0.0; // Keep movement on horizontal plane (no flying)

    // Apply movement with time-based speed
    // 📋 PERFORMANCE NOTE: normalize_or_zero prevents faster diagonal movement
    player.translation += to_move.normalize_or_zero() * time.delta_secs() * MOVEMENT_SPEED;
}

// ============================================================================
// 👀 SECTION 3: MOUSE LOOK (Rotation Control)
// ============================================================================

/// 🎯 SYSTEM 2: PLAYER LOOK
/// Handles mouse input for player rotation (looking around)
///
/// 📋 BEST PRACTICE: Mouse look implementation
/// - Check window focus to prevent unwanted rotation
/// - Scale sensitivity based on window size for consistency
/// - Clamp pitch to prevent camera flipping
fn player_look(
    mut player: Single<&mut Transform, With<Player>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    // Don't process mouse input if window isn't focused
    // 📋 BEST PRACTICE: Always check focus for mouse input
    if !window.focused {
        return;
    }

    let dt = time.delta_secs();
    // Calculate sensitivity based on window size for consistent feel
    // 📋 DESIGN NOTE: Smaller windows need higher sensitivity to feel the same
    let sensitivity = 1.0 * 100.0 / window.width().min(window.height());

    // Convert rotation to Euler angles for easier manipulation
    use EulerRot::YXZ;
    let (mut yaw, mut pitch, _) = player.rotation.to_euler(YXZ);
    
    // Apply mouse movement to rotation
    yaw -= mouse_motion.delta.x * dt * sensitivity;   // Horizontal mouse = yaw rotation
    pitch -= mouse_motion.delta.y * dt * sensitivity; // Vertical mouse = pitch rotation
    
    // Clamp pitch to prevent flipping upside down
    // 📋 BEST PRACTICE: Always clamp pitch for better user experience
    pitch = pitch.clamp(-1.57, 1.57); // Roughly -90° to +90°

    // Apply new rotation back to player
    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.0);
}

// ============================================================================
// 🖱️ SECTION 4: CURSOR MANAGEMENT (Grab/Release)
// ============================================================================

/// 🎯 OBSERVER: CURSOR GRAB APPLICATION
/// Applies cursor grab/release state to the window
///
/// 📋 BEST PRACTICE: Use observers for event handling
/// - Clean separation between event triggering and handling
/// - Multiple systems can trigger the same event
/// - Observer pattern is more flexible than direct function calls
fn apply_grab(grab: Trigger<GrabEvent>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    use bevy::window::CursorGrabMode;
    
    if **grab {
        // Grab cursor for immersive gameplay
        window.cursor_options.visible = false;           // Hide cursor
        window.cursor_options.grab_mode = CursorGrabMode::Locked; // Lock to window
    } else {
        // Release cursor for UI interaction
        window.cursor_options.visible = true;            // Show cursor
        window.cursor_options.grab_mode = CursorGrabMode::None;   // Allow free movement
    }
}

/// 🎯 SYSTEM 3: WINDOW FOCUS HANDLING
/// Automatically manages cursor grab based on window focus
///
/// 📋 BEST PRACTICE: Handle focus changes gracefully
/// - Grab cursor when window gains focus (immersive gameplay)
/// - Release cursor when window loses focus (prevents trapping)
/// - Use events to keep systems decoupled
use bevy::window::WindowFocused;
fn focus_event(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        // Trigger grab event based on focus state
        commands.trigger(GrabEvent(event.focused));
    }
}

/// 🎯 SYSTEM 4: MANUAL CURSOR TOGGLE
/// Allows player to manually toggle cursor grab with Escape key
///
/// 📋 BEST PRACTICE: Give player control over cursor
/// - Escape key is standard for releasing cursor in games
/// - Toggle behavior feels natural to players
/// - Updates window focus state to match cursor state
fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    // Toggle focus state (which affects cursor grab through focus_event)
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused));
}
