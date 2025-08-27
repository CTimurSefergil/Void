use super::super::components::{EmotionalState, OInsanAI};
use bevy::prelude::*;

pub fn ai_emotion_system(mut ai_query: Query<&mut OInsanAI>, _time: Res<Time>) {
    for mut ai in ai_query.iter_mut() {
        let new_emotion = calculate_emotion_from_health(ai.health, ai.max_health);

        if new_emotion != ai.emotional_state {
            log_emotion_change(&ai.emotional_state, &new_emotion, ai.health, ai.max_health);
            ai.emotional_state = new_emotion;
        }
    }
}

fn calculate_emotion_from_health(health: f32, max_health: f32) -> EmotionalState {
    let health_percent = health / max_health;
    
    match health_percent {
        hp if hp <= 0.3 => EmotionalState::Depressed,
        hp if hp >= 0.7 => EmotionalState::Angry,
        _ => EmotionalState::Neutral,
    }
}

fn log_emotion_change(
    old_emotion: &EmotionalState,
    new_emotion: &EmotionalState,
    current_health: f32,
    max_health: f32,
) {
    let health_percent = (current_health / max_health) * 100.0;
    println!(
        "🧠 AI emotion: {:?} → {:?} (Health: {:.0}%)",
        old_emotion, new_emotion, health_percent
    );
}
