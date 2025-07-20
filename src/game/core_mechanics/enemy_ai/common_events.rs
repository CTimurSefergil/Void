use bevy::prelude::*;

// AI EVENTS
#[derive(Event)]
pub struct _PlayerSeen;

#[derive(Event)]
pub struct _PlayerHeard;

#[derive(Event)]
pub struct PlayerInRange {
    pub monster: Entity,
    pub last_seen: i32
}

#[derive(Event)]
pub struct _Suspicious;

#[derive(Event)]
pub struct Depressed;
