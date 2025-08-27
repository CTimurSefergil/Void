use bevy::prelude::*;

use crate::game::core_mechanics::enemy_ai::o_insan::spawn::spawn_o_insan;

pub mod components;
pub mod debug;
pub mod spawn;
pub mod systems;

pub struct SimpleAIPlugin;

impl Plugin for SimpleAIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ai_demo)
            .add_systems(
                Update,
                (
                    systems::ai_emotion_system,  
                    systems::ai_behavior_system, 
                    systems::ai_movement_system, 
                    systems::ai_speech_system,   
                    systems::ai_health_system,   
                    debug::ai_debug_system,      
                )
                    .chain(),
            ); 
    }
}

fn setup_ai_demo(
    mut commands: Commands,
    mesh_assets: ResMut<Assets<Mesh>>,
    material_assets: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let _ai_entity = spawn_o_insan(
        &mut commands,
        Vec3::new(0.0, 0.0, 0.0),
        100.0,
        mesh_assets,
        material_assets,
        asset_server,
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
