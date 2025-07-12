use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Memory {
    is_player_seen: bool,
    is_attacking: bool,
    depression: f32,
}