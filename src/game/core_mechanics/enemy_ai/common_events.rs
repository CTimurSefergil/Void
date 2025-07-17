use bevy::prelude::*;

// AI EVENTS
#[derive(Event)]
pub struct PlayerSeen;

#[derive(Event)]
pub struct PlayerHeard;

#[derive(Event)]
pub struct PlayerInRange {
    pub monster: Entity,
    pub last_seen: i32
}

#[derive(Event)]
pub struct Suspicious;

#[derive(Event)]
pub struct Depressed;
