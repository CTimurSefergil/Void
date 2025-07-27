// ============================================================================
// 🏭 SECTION 6: FACTORY FUNCTIONS (Creating Entities)
// ============================================================================

use super::components::OInsanAI;
use bevy::prelude::*;

/// Creates a new AI entity with all necessary components
///
/// 📋 BEST PRACTICE: Use factory functions
/// - Centralizes entity creation
/// - Ensures all required components are added
/// - Makes it easy to create multiple AIs with different settings
pub fn spawn_simple_ai(
    commands: &mut Commands,
    position: Vec3,
    health: f32,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let ball_mesh = mesh_assets.add(Cuboid::default());
    let color = Color::srgb(0.8, 0.5, 0.6);
    let ball_material = material_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });
    commands
        .spawn((
            OInsanAI {
                health,
                max_health: health,
                ..Default::default()
            },
            Mesh3d(ball_mesh),
            MeshMaterial3d(ball_material.clone()),
            Transform::from_translation(position),
            Name::new("TheHuman"),
        ))
        .id()
}
