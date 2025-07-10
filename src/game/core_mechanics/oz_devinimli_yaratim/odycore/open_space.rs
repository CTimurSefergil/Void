use std::collections::VecDeque;

use bevy::ecs::{
    entity::Entity,
    query::Added,
    resource::Resource,
    system::{Query, Res, ResMut},
};
use rand::seq::IteratorRandom;

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    cells::{Cell, CellSpatialIndex},
    odyrules::{
        commons::{DIRECTION_VECTORS, Direction, Rules, TileType},
        open_space_rules::OpenSpaceRules,
    },
};

#[derive(Resource, Default)]
pub struct OpenSpacePropagationQueue {
    pub queue: VecDeque<Entity>,
}

pub fn initialize_new_cells(
    mut wfc_queue: ResMut<OpenSpacePropagationQueue>,
    added_cells: Query<&Cell, Added<Cell>>,
    spatial_index: Res<CellSpatialIndex>,
    cells: Query<&Cell>,
) {
    for cell in added_cells.iter() {
        for (_, (dx, dz)) in DIRECTION_VECTORS.iter() {
            let neighbor_pos = (cell.position.0 + dx, cell.position.1 + dz);
            if let Some(neighbor_entity) = spatial_index.grid.get(&neighbor_pos) {
                if let Ok(neighbor_cell) = cells.get(*neighbor_entity) {
                    if neighbor_cell.is_collapsed {
                        wfc_queue.queue.push_front(*neighbor_entity);
                    }
                }
            }
        }
    }
}

pub fn filter_valid_tiles<T>(
    valid_tiles: &mut Vec<TileType>,
    neighbor_tile: TileType,
    direction: Direction,
    rules: &T,
) where
    T: Rules,
{
    if let Some(allowed_for_direction) = rules.allowed_neighbors().get(&neighbor_tile) {
        if let Some(allowed_tiles) = allowed_for_direction.get(&direction) {
            valid_tiles.retain(|tile| allowed_tiles.contains(tile));
        }
    }
}

pub fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Front => Direction::Back,
        Direction::Back => Direction::Front,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
    }
}

pub fn propagate_open_space_constraints(
    mut open_space: ResMut<OpenSpacePropagationQueue>,
    rules: Res<OpenSpaceRules>,
    spatial_index: Res<CellSpatialIndex>,
    mut cells: Query<&mut Cell>,
) {
    while let Some(entity) = open_space.queue.pop_front() {
        let (is_collapsed, tile_type, position) = {
            if let Ok(cell) = cells.get_mut(entity) {
                (cell.is_collapsed, cell.tile_type, cell.position)
            } else {
                continue;
            }
        };

        if is_collapsed {
            if let Some(tile) = tile_type {
                for (direction, (dx, dz)) in DIRECTION_VECTORS.iter() {
                    let neighbor_pos = (position.0 + dx, position.1 + dz);

                    if let Some(neighbor_entity) = spatial_index.grid.get(&neighbor_pos) {
                        if let Ok(mut neighbor_cell) = cells.get_mut(*neighbor_entity) {
                            if neighbor_cell.is_collapsed {
                                continue;
                            }
                            filter_valid_tiles(
                                &mut neighbor_cell.valid_tiles,
                                tile,
                                get_opposite_direction(*direction),
                                rules.as_ref(),
                            );
                            neighbor_cell.update_entropy();

                            if neighbor_cell.is_contradicted() {
                                println!("lv u");
                                neighbor_cell.tile_type = Some(TileType::Ground);
                                neighbor_cell.is_collapsed = true;
                                neighbor_cell.entropy = 0;
                                open_space.queue.push_back(*neighbor_entity);
                            }
                        }
                    }
                }
            }
        }
    }
}

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

pub fn collapse_lowest_entropy_open_space_cell(
    mut open_space: ResMut<OpenSpacePropagationQueue>,
    mut cells: Query<(Entity, &mut Cell)>,
    open_space_rules: Res<OpenSpaceRules>,
) {
    let mut candidates = cells
        .iter_mut()
        .filter(|(_, cell)| !cell.is_collapsed)
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        return;
    }

    let min_entropy = candidates
        .iter()
        .map(|(_, cell)| cell.entropy)
        .min()
        .unwrap();

    candidates.retain(|(_, cell)| cell.entropy == min_entropy);

    if let Some((entity, cell)) = candidates
        .into_iter()
        .choose(&mut rand::rng())
        .map(|(e, c)| (e, c.into_inner()))
    {
        if !cell.valid_tiles.is_empty() {
            let tile = get_random_tile(open_space_rules.as_ref(), &cell.valid_tiles);

            cell.tile_type = Some(tile);
            cell.is_collapsed = true;
            cell.entropy = 0;
            open_space.queue.push_back(entity);
        }
    }
}
