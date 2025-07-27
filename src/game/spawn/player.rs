// ============================================================================
// 👤 PLAYER SYSTEM - Player Entity Creation and Management
// ============================================================================
//
// This module handles everything related to the player entity:
// - Player component definition
// - Player spawning logic
// - Player-specific data management
//
// 📋 BEST PRACTICE: Component-driven entity design
// - Define clear component structures
// - Use Default trait for sensible defaults
// - Reflect trait enables debugging and editor support

use bevy::prelude::*;

/// 🎯 PLUGIN SETUP: Player System Registration
/// Adds player spawning system to run at game startup
///
/// 📋 BEST PRACTICE: Use Startup systems for initial entity creation
/// - Startup systems run once when the game begins
/// - Perfect for creating essential entities like the player
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
}

// ============================================================================
// 📊 SECTION 1: PLAYER COMPONENT (Data Structure)
// ============================================================================

/// The main player component that defines player-specific data
///
/// 📋 BEST PRACTICE: Keep components focused and simple
/// - Only store data that's specific to the player
/// - Use bool for simple state flags
/// - Derive useful traits for debugging and reflection
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)] // Enables Bevy editor support and runtime inspection
pub struct Player {
    /// Whether the player currently has a weapon equipped
    ///
    /// 📋 DESIGN NOTE: This affects AI behavior
    /// - AI becomes sorrowful when they see an armed player while chasing
    /// - Simple boolean keeps the logic straightforward
    pub has_weapon: bool,
}

impl Default for Player {
    /// 🎯 DEFAULT IMPLEMENTATION: Sensible Starting Values
    ///
    /// 📋 BEST PRACTICE: Always provide sensible defaults
    /// - New players start without weapons (peaceful beginning)
    /// - Default values should represent the most common initial state
    fn default() -> Self {
        Self {
            has_weapon: false, // Players start peaceful, can acquire weapons later
        }
    }
}

// ============================================================================
// 🏭 SECTION 2: PLAYER SPAWNING (Entity Creation)
// ============================================================================

/// 🎯 SYSTEM 1: PLAYER SPAWNING
/// Creates the player entity with all necessary components
///
/// 📋 BEST PRACTICE: Complete entity setup in one place
/// - Add all required components together
/// - Set reasonable initial transform values
/// - Use consistent scaling and positioning
fn spawn_player(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    // Create player geometry (currently commented out for invisible player)
    let _ball_mesh = mesh_assets.add(Extrusion::new(Annulus::new(14.0, 15.0), 20.0));
    let color = Color::srgb(0.05, 0.5, 0.6); // Bluish-teal color
    let _ball_material = material_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });

    // Spawn the player entity with essential components
    commands.spawn((
        // Transform: Position, rotation, and scale in 3D space
        Transform::from_translation(Vec3 {
            x: 0.0, // Center of world
            y: 2.0, // Slightly above ground level
            z: 0.0, // Center of world
        })
        .with_scale(Vec3 {
            x: 5.0, // Large scale for visibility
            y: 5.0,
            z: 5.0,
        }),
        // Visibility: Controls whether the entity is rendered
        // 📋 DESIGN NOTE: Player might be invisible for first-person feel
        Visibility::default(),
        // Player component: Our custom player data
        Player::default(),
    ));

    // 📋 COMMENTED OUT: Visual mesh rendering
    // The mesh creation code is commented out, suggesting the player
    // might be intended to be invisible (first-person style)
    /*
    .with_children(|parent| {
        parent.spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_x(0.5 * std::f32::consts::PI)),
            Visibility::default(),
            Mesh3d(ball_mesh.clone()),
            MeshMaterial3d(ball_material),
        ));
    })
    */
}
