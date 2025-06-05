use bevy::prelude::*;

use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::{Direction, TileType, WFCRules};

pub fn filter_valid_tiles(
    valid_tiles: &mut Vec<TileType>,
    neighbor_tile: TileType,
    direction: Direction,
    rules: &WFCRules,
) {
    let opposite_direction = get_opposite_direction(direction);

    if let Some(allowed_for_direction) = rules.allowed_neighbors.get(&neighbor_tile) {
        if let Some(allowed_tiles) = allowed_for_direction.get(&opposite_direction) {
            valid_tiles.retain(|tile| allowed_tiles.contains(tile));
        }
    }
}

pub fn get_random_tile(rules: &WFCRules, valid_tiles: &[TileType]) -> TileType {
    use rand::prelude::*;

    if valid_tiles.is_empty() {
        return TileType::Ground;
    }

    let mut rng = rand::rng();
    let total_weight: f32 = valid_tiles
        .iter()
        .map(|t| *rules.weights.get(t).unwrap_or(&1.0))
        .sum();

    let mut random = rng.random_range(0.0..total_weight);
    for &tile in valid_tiles {
        let weight = *rules.weights.get(&tile).unwrap_or(&1.0);
        if random <= weight {
            return tile;
        }
        random -= weight;
    }

    valid_tiles[0]
}

pub fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Front => Direction::Back,
        Direction::Back => Direction::Front,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
    }
}
