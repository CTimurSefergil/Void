use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmotionalState {
    Depressed, 
    Angry,     
    Neutral,   
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIBehavior {
    Wandering, 
    Chasing,   
    Escaping, 
    Begging,   
}

#[derive(Component)]
pub struct OInsanAI {
    pub health: f32,                  
    pub max_health: f32,                
    pub emotional_state: EmotionalState,
    pub current_behavior: AIBehavior,  

    pub behavior_update_timer: Timer, 
    pub speech_timer: Timer,         

    pub last_player_position: Option<Vec3>, 
    pub time_since_seen_player: f32,      


    pub movement_speed: f32,
    pub detection_range: f32,
}

impl Default for OInsanAI {
    fn default() -> Self {
        Self {
            health: 100.0,
            max_health: 100.0,
            emotional_state: EmotionalState::Neutral,
            current_behavior: AIBehavior::Wandering,

            behavior_update_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            speech_timer: Timer::from_seconds(2.0, TimerMode::Repeating),

            last_player_position: None,
            time_since_seen_player: 0.0,

            movement_speed: 10.0,
            detection_range: 40.0,
        }
    }
}
