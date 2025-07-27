// ============================================================================
// 📷 CAMERA SYSTEM - Third-Person Following Camera
// ============================================================================
//
// This module handles all camera behavior in the Void game.
// The camera follows the player with smooth interpolation and responds to mouse input.
//
// 📋 BEST PRACTICE: Separate camera logic from player logic
// - Camera systems are independent of player systems
// - Smooth following prevents jarring camera movements
// - Mouse sensitivity is calculated based on window size for consistency

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::pbr::ClusterConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::spawn::player::Player;

/// 🎯 PLUGIN SETUP: Camera System Registration
/// Registers all camera-related systems with Bevy
///
/// 📋 BEST PRACTICE: Use plugin pattern for system organization
/// - Startup systems run once when the game starts
/// - Update systems run every frame
/// - Logical grouping makes code easier to maintain
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, (update_camera, camera_look));
}

// ============================================================================
// 🏗️ SECTION 1: CAMERA CREATION (Initial Setup)
// ============================================================================

/// 🎯 SYSTEM 1: CAMERA SPAWNING
/// Creates the main camera and lighting for the game world
///
/// 📋 BEST PRACTICE: Set up essential components together
/// - DirectionalLight provides global lighting
/// - Camera3d gives us 3D perspective
/// - ClusterConfig::Single optimizes lighting for single light source
fn spawn_camera(mut commands: Commands) {
    // Add global directional lighting to the scene
    commands.spawn(DirectionalLight::default());
    
    // Create the main 3D camera
    commands.spawn((
        Name::new("Camera"), // Useful for debugging and hierarchy viewing
        Camera3d::default(),
        Camera {
            // Uncomment below for custom clear color (greenish background)
            //clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.6, 0.15)),
            ..Default::default()
        },
        IsDefaultUiCamera, // This camera also handles UI rendering
        ClusterConfig::Single, // Optimize lighting calculations for single light
    ));
}

// ============================================================================
// 🎮 SECTION 2: CAMERA MOVEMENT (Following the Player)
// ============================================================================

/// 🎯 SYSTEM 2: CAMERA FOLLOWING
/// Makes the camera smoothly follow the player with a fixed offset
///
/// 📋 BEST PRACTICE: Use lerp for smooth camera movement
/// - Prevents jarring camera jumps when player moves quickly
/// - Y-offset keeps camera above player for better view
/// - Time-based lerp ensures consistent speed regardless of framerate
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera3d>)>,
    time: Res<Time>,
) {
    // Calculate target camera position (3 units above player)
    let target_position = Vec3 {
        x: player.translation.x,
        y: player.translation.y + 3.0, // Fixed height offset
        z: player.translation.z,
    };

    // Smoothly interpolate to target position
    // 📋 PERFORMANCE NOTE: Using lerp with time.delta_secs() ensures smooth movement
    camera.translation = camera.translation.lerp(target_position, time.delta_secs() * 2.0);
}

// ============================================================================
// 🖱️ SECTION 3: MOUSE LOOK (Camera Rotation)
// ============================================================================

/// 🎯 SYSTEM 3: MOUSE LOOK CONTROL
/// Allows player to look around with mouse movement
///
/// 📋 BEST PRACTICE: Handle input gracefully
/// - Check window focus to prevent unwanted camera movement
/// - Clamp pitch to prevent camera flipping
/// - Scale sensitivity based on window size for consistent feel
fn camera_look(
    mut camera: Single<&mut Transform, With<IsDefaultUiCamera>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    // Don't process mouse input if window isn't focused
    // 📋 BEST PRACTICE: Always check window focus for input systems
    if !window.focused {
        return;
    }

    let dt = time.delta_secs();
    // Calculate sensitivity based on window size for consistent feel across resolutions
    let sensitivity = 1.0 * 100.0 / window.width().min(window.height());

    // Convert current rotation to Euler angles for easier manipulation
    use EulerRot::YXZ;
    let (mut yaw, mut pitch, _) = camera.rotation.to_euler(YXZ);
    
    // Apply mouse movement to rotation
    // 📋 PERFORMANCE NOTE: Negative values because mouse Y is inverted
    yaw -= mouse_motion.delta.x * dt * sensitivity;
    pitch -= mouse_motion.delta.y * dt * sensitivity;
    
    // Clamp pitch to prevent camera from flipping upside down
    // 📋 BEST PRACTICE: Always clamp camera pitch for better user experience
    pitch = pitch.clamp(-1.57, 1.57); // Roughly -90° to +90°

    // Apply the new rotation back to the camera
    camera.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.0);
}
