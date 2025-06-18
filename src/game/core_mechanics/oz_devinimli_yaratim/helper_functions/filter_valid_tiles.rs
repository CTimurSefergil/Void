use crate::game::core_mechanics::oz_devinimli_yaratim::{
    helper_functions::get_opposite_direction::get_opposite_direction,
    odyrules::commons::{Direction, Rules, TileType},
};

pub fn filter_valid_tiles<T>(
    valid_tiles: &mut Vec<TileType>,
    neighbor_tile: TileType,
    direction: Direction,
    rules: &T,
) where
    T: Rules,
{
    let opposite_direction = get_opposite_direction(direction);

    if let Some(allowed_for_direction) = rules.allowed_neighbors().get(&neighbor_tile) {
        if let Some(allowed_tiles) = allowed_for_direction.get(&opposite_direction) {
            valid_tiles.retain(|tile| allowed_tiles.contains(tile));
        }
    }
}
