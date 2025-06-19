use std::collections::VecDeque;

use bevy::{
    ecs::{
        entity::Entity,
        resource::Resource,
        system::{Query, Res, ResMut},
    },
    platform::collections::HashSet,
};
use rand::seq::IteratorRandom;

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    helper_functions::{filter_valid_tiles::filter_valid_tiles, get_random_tile::get_random_tile},
    odycore::cell::{Cell, CellSpatialIndex},
    odyrules::{
        commons::{DIRECTION_VECTORS, TileType},
        open_space_rules::OpenSpaceRules,
    },
};

#[derive(Resource, Default)]
pub struct OpenSpacePropagationQueue {
    pub queue: VecDeque<Entity>,
}

pub fn collapse_lowest_entropy_open_space_cell(
    mut open_space_queue: ResMut<OpenSpacePropagationQueue>,
    mut cells: Query<(Entity, &mut Cell)>,
    open_space_rules: Res<OpenSpaceRules>,
) {
    // PSEUDO CODE for collapse_lowest_entropy_cell function:

    // 1. If propagation queue is not empty, return early (propagation has priority)
    // 2. Collect all uncollapsed cells as candidates
    // 3. If no candidates exist, return (all cells are collapsed)
    // 4. Find the minimum entropy value among all candidates
    // 5. Filter candidates to only include those with minimum entropy
    // 6. Randomly select one candidate from the filtered list
    // 7. If selected cell has valid tiles:
    //    a. Choose a random tile from valid tiles using rules
    //    b. Set cell as collapsed with chosen tile
    //    c. Reset entropy to 0
    //    d. Add entity to propagation queue for constraint propagation

    if !open_space_queue.queue.is_empty() {
        return;
    }

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
        .unwrap_or(i32::MAX);

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

            open_space_queue.queue.push_back(entity);
        }
    }
}

pub fn propagate_open_space_constraints(
    mut wfc_queue: ResMut<OpenSpacePropagationQueue>,
    rules: Res<OpenSpaceRules>,
    spatial_index: Res<CellSpatialIndex>,
    mut cells: Query<&mut Cell>,
) {
    // PSEUDO CODE for propagate_constraints function:

    // 1. Initialize a set to track processed entities (avoid infinite loops)
    // 2. While there are entities in the propagation queue:
    //    a. Pop the front entity from queue
    //    b. Skip if already processed in this iteration
    //    c. Mark entity as processed
    //
    //    d. Extract cell data (collapsed state, tile type, position, valid tiles)
    //    e. If entity no longer exists, continue to next
    //
    //    f. If cell is collapsed:
    //       - For each direction (North, South, East, West):
    //         * Calculate neighbor position
    //         * If neighbor exists and is not collapsed:
    //           - Filter neighbor's valid tiles based on current tile's constraints
    //           - If neighbor's valid tiles changed, add neighbor to queue
    //           - If neighbor has contradiction (no valid tiles), reset to Ground
    //
    //    g. If cell is not collapsed:
    //       - Create copy of current valid tiles
    //       - For each direction:
    //         * Check if neighbor exists and is collapsed
    //         * If so, filter current cell's valid tiles based on neighbor's constraints
    //       - If valid tiles changed:
    //         * Update cell's valid tiles and entropy
    //         * If contradiction occurs, reset to Ground tile

    let mut processed = HashSet::new();

    while let Some(entity) = wfc_queue.queue.pop_front() {
        if processed.contains(&entity) {
            continue;
        }
        processed.insert(entity);

        let (is_collapsed, tile_type, position, valid_tiles) = {
            if let Ok(cell) = cells.get(entity) {
                (
                    cell.is_collapsed,
                    cell.tile_type,
                    cell.position,
                    cell.valid_tiles.clone(),
                )
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

                            let prev_valid_count = neighbor_cell.valid_tiles.len();
                            filter_valid_tiles(
                                &mut neighbor_cell.valid_tiles,
                                tile,
                                *direction,
                                rules.as_ref(),
                            );

                            if neighbor_cell.valid_tiles.len() != prev_valid_count {
                                wfc_queue.queue.push_back(*neighbor_entity);
                            }

                            if neighbor_cell.is_contradiction() {
                                neighbor_cell.valid_tiles = vec![TileType::Empty];
                                neighbor_cell.entropy = 1;
                                wfc_queue.queue.push_back(*neighbor_entity);
                            }
                        }
                    }
                }
            }
        } else {
            let mut new_valid_tiles = valid_tiles;
            let mut changed = false;

            for (direction, (dx, dz)) in DIRECTION_VECTORS.iter() {
                let neighbor_pos = (position.0 + dx, position.1 + dz);

                if let Some(neighbor_entity) = spatial_index.grid.get(&neighbor_pos) {
                    if let Ok(neighbor_cell) = cells.get(*neighbor_entity) {
                        if neighbor_cell.is_collapsed {
                            if let Some(neighbor_tile) = neighbor_cell.tile_type {
                                let prev_count = new_valid_tiles.len();
                                filter_valid_tiles(
                                    &mut new_valid_tiles,
                                    neighbor_tile,
                                    *direction,
                                    rules.as_ref(),
                                );
                                if new_valid_tiles.len() != prev_count {
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }

            if changed {
                if let Ok(mut cell) = cells.get_mut(entity) {
                    cell.valid_tiles = new_valid_tiles;
                    cell.entropy = cell.valid_tiles.len() as i32;

                    if cell.is_contradiction() {
                        cell.valid_tiles = vec![TileType::Empty];
                        cell.entropy = 1;
                    }
                }
            }
        }
    }
}
