use super::super::components::{AIBehavior, OInsanAI};
use crate::game::spawn::player::Player;
use bevy::prelude::*;
use rand::{prelude::*, rng};

pub fn ai_movement_system(
    mut ai_query: Query<(&mut Transform, &OInsanAI), Without<Player>>,
    player_query: Query<&Transform, (With<Player>, Without<OInsanAI>)>,
    time: Res<Time>,
) {
    for (mut ai_transform, ai) in ai_query.iter_mut() {
        let delta_time = time.delta_secs();
        let base_movement_distance = ai.movement_speed * delta_time;

        execute_behavior_movement(
            &mut ai_transform,
            ai,
            &player_query,
            base_movement_distance,
        );
    }
}

fn execute_behavior_movement(
    ai_transform: &mut Transform,
    ai: &OInsanAI,
    player_query: &Query<&Transform, (With<Player>, Without<OInsanAI>)>,
    base_movement_distance: f32,
) {
    match ai.current_behavior {
        AIBehavior::Wandering => execute_wandering_movement(ai_transform, base_movement_distance),
        AIBehavior::Chasing => execute_chasing_movement(ai_transform, player_query, base_movement_distance),
        AIBehavior::Escaping => execute_escaping_movement(ai_transform, player_query, ai, base_movement_distance),
        AIBehavior::Begging => execute_begging_movement(ai_transform, player_query),
    }
}

fn execute_wandering_movement(ai_transform: &mut Transform, base_movement_distance: f32) {
    let mut rng = rng();
    let random_direction = Vec3::new(
        rng.random_range(-10.0..10.0),
        0.0, 
        rng.random_range(-10.0..10.0),
    )
    .normalize_or_zero();

    let wandering_speed_multiplier = 0.5;
    ai_transform.translation += random_direction * base_movement_distance * wandering_speed_multiplier;
}

fn execute_chasing_movement(
    ai_transform: &mut Transform,
    player_query: &Query<&Transform, (With<Player>, Without<OInsanAI>)>,
    base_movement_distance: f32,
) {
    if let Ok(player_transform) = player_query.single() {
        let direction = (player_transform.translation - ai_transform.translation)
            .normalize_or_zero();
        
        ai_transform.translation += direction * base_movement_distance;
        
        if direction != Vec3::ZERO {
            ai_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

fn execute_escaping_movement(
    ai_transform: &mut Transform,
    player_query: &Query<&Transform, (With<Player>, Without<OInsanAI>)>,
    ai: &OInsanAI,
    base_movement_distance: f32,
) {
    let escape_direction = calculate_escape_direction(ai_transform, player_query, ai);
    
    if let Some(direction) = escape_direction {
        let escape_speed_multiplier = 1.5;
        ai_transform.translation += direction * base_movement_distance * escape_speed_multiplier;
    }
}

fn calculate_escape_direction(
    ai_transform: &Transform,
    player_query: &Query<&Transform, (With<Player>, Without<OInsanAI>)>,
    ai: &OInsanAI,
) -> Option<Vec3> {
    if let Ok(player_transform) = player_query.single() {
        let escape_direction = (ai_transform.translation - player_transform.translation)
            .normalize_or_zero();
        return Some(escape_direction);
    }
    
    if let Some(last_player_pos) = ai.last_player_position {
        let escape_direction = (ai_transform.translation - last_player_pos)
            .normalize_or_zero();
        return Some(escape_direction);
    }
    
    None
}

fn execute_begging_movement(
    ai_transform: &mut Transform,
    player_query: &Query<&Transform, (With<Player>, Without<OInsanAI>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        ai_transform.look_at(player_transform.translation, Vec3::Y);
    }
}
