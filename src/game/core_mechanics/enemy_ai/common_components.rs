use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

// AI COMPONENTS
#[derive(Component)]
pub struct PlayerSeen(pub bool);

#[derive(Component)]
pub struct PlayerHeard(pub bool);

#[derive(Component)]
pub struct PlayerInRange(pub bool);

#[derive(Component)]
pub struct Suspicious(pub bool);

#[derive(Component)]
pub struct Depressed(pub bool);
