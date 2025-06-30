use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::{Rules, TileType};

pub fn get_random_tile<T>(rules: &T, valid_tiles: &[TileType]) -> TileType
where
    T: Rules,
{
    use rand::prelude::*;

    if valid_tiles.is_empty() {
        return TileType::Ground;
    }

    let mut rng = rand::rng();
    let total_weight: f32 = valid_tiles
        .iter()
        .map(|t| *rules.weights().get(t).unwrap_or(&1.0))
        .sum();

    let mut random = rng.random_range(0.0..total_weight);
    for &tile in valid_tiles {
        let weight = *rules.weights().get(&tile).unwrap_or(&1.0);
        random -= weight;
        if random <= 0.0 {
            return tile;
        }
    }

    valid_tiles[0]
}
