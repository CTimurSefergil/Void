use super::super::components::{AIBehavior, EmotionalState, OInsanAI};
use bevy::prelude::*;
use rand::{prelude::*, rng};

pub fn ai_speech_system(mut ai_query: Query<&mut OInsanAI>, time: Res<Time>) {
    for mut ai in ai_query.iter_mut() {
        ai.speech_timer.tick(time.delta());

        if ai.speech_timer.just_finished() {
            attempt_speech(&ai);
        }
    }
}

fn attempt_speech(ai: &OInsanAI) {
    let mut rng = rng();
    
    if !rng.random_bool(0.8) {
        return;
    }

    let dialogue_options = select_dialogue_for_state(ai.emotional_state, ai.current_behavior);
    
    if !dialogue_options.is_empty() {
        let selected_line = dialogue_options[rng.random_range(0..dialogue_options.len())];
        println!("🤖 AI says: '{}'", selected_line);
    }
}

fn select_dialogue_for_state(
    emotional_state: EmotionalState,
    current_behavior: AIBehavior,
) -> Vec<&'static str> {
    match (emotional_state, current_behavior) {
        (EmotionalState::Depressed, AIBehavior::Escaping) => vec![
            "Leave me alone...",
            "I can't take this anymore...",
            "Why won't you just go away?",
            "Please, I just want to be left in peace...",
            "I'm too tired for this...",
        ],

        (EmotionalState::Depressed, AIBehavior::Wandering) => vec![
            "Everything hurts...",
            "What's the point anymore?",
            "I feel so empty...",
            "Is this all there is?",
        ],

        (EmotionalState::Angry, AIBehavior::Chasing) => vec![
            "Come here!",
            "You can't escape me!",
            "I'm going to get you!",
            "Stop running!",
            "You're mine now!",
        ],

        (EmotionalState::Angry, AIBehavior::Begging) => vec![
            "I... I love you...",
            "Please don't hurt me...",
            "I don't want to fight anymore...",
            "That weapon scares me...",
        ],

        (EmotionalState::Angry, AIBehavior::Wandering) => vec![
            "Where did you go?",
            "I know you're out there...",
            "Come out and face me!",
            "Hiding won't save you...",
        ],

        (EmotionalState::Neutral, AIBehavior::Wandering) => vec![
            "Where am I?",
            "Just walking around...",
            "Nothing to do...",
            "Peaceful day...",
            "Wonder what's over there...",
        ],

        (EmotionalState::Neutral, AIBehavior::Begging) => vec![
            "I love you...",
            "You seem nice...",
            "Want to be friends?",
            "I like your company...",
            "Stay with me...",
        ],

        _ => vec!["..."],
    }
}
