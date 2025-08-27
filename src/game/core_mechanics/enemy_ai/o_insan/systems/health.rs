use super::super::components::OInsanAI;
use bevy::prelude::*;

pub fn ai_health_system(mut ai_query: Query<&mut OInsanAI>) {
    for mut ai in ai_query.iter_mut() {
        let previous_health = ai.health;
        
        ai.health = ai.health.clamp(0.0, ai.max_health);

        handle_health_state_changes(&ai, previous_health);
    }
}

fn handle_health_state_changes(ai: &OInsanAI, previous_health: f32) {
    if previous_health > 0.0 && ai.health <= 0.0 {
        handle_death_event();
    }
    
    if previous_health <= 0.0 && ai.health > 0.0 {
        handle_revival_event(ai.health);
    }
    
    let health_percent = ai.health / ai.max_health;
    if health_percent <= 0.1 && previous_health / ai.max_health > 0.1 {
        handle_critical_health_event(health_percent);
    }
}

fn handle_death_event() {
    println!("💀 AI has died!");
}

fn handle_revival_event(new_health: f32) {
    println!("✨ AI has been revived! (Health: {:.1})", new_health);
}

fn handle_critical_health_event(health_percent: f32) {
    println!(
        "⚠️ AI health critical! ({:.0}%)",
        health_percent * 100.0
    );
}
