use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerInRange {
    pub monster: Entity,
    pub last_seen: i32,
}

#[derive(Event)]
pub struct Suspicious;

#[derive(Event)]
pub struct Depressed;

#[derive(Component, Debug)]
pub enum DepressionStates {
    Low,
    Normal,
    High,
    Extreme,
    NeverSeenBefore,
    Wtf,
    Minel,
    Ege,
}

#[derive(Component)]
pub struct Health {
    pub health: i32,
}
