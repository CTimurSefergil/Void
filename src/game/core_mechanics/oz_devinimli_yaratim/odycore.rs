use bevy::{platform::collections::HashSet, prelude::*};
use rand::seq::IteratorRandom;
use std::collections::{HashMap, VecDeque};

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    helper_functions::{filter_valid_tiles, get_random_tile},
    odyrules::{
        commons::{DIRECTION_VECTORS, TileType},
        open_space_rules::OpenSpaceRules,
    },
};

pub fn plugin(app: &mut App) {
    app.init_resource::<OpenSpaceRules>()
        .init_resource::<CellSpatialIndex>()
        .init_resource::<RegularPropagationQueue>()
        .init_resource::<BuildingPropagationQueue>()
        .add_systems(Startup, setup_wfc_rules)
        .add_systems(
            Update,
            (
                update_spatial_index,
                initialize_new_cells,
                propagate_building_constraints,
                propagate_regular_constraints,
                collapse_lowest_entropy_cell,
            )
                .chain(),
        );
}

fn setup_wfc_rules(mut commands: Commands) {
    commands.insert_resource(OpenSpaceRules::default());
    commands.insert_resource(CellSpatialIndex::default());
    commands.insert_resource(RegularPropagationQueue::default());
}

#[derive(Resource, Default)]
pub struct CellSpatialIndex {
    pub grid: HashMap<(i32, i32), Entity>,
}

#[derive(Resource, Default)]
pub struct RegularPropagationQueue {
    pub queue: VecDeque<Entity>,
}

#[derive(Resource, Default)]
pub struct BuildingPropagationQueue {
    pub queue: VecDeque<Entity>,
}

#[derive(Resource, Default)]
pub struct DungeonPropagationQueue {
    pub queue: VecDeque<Entity>,
}

#[derive(Component, Debug)]
pub struct Cell {
    pub is_collapsed: bool,

    pub tile_type: Option<TileType>,
    pub entropy: i32,
    pub valid_tiles: Vec<TileType>,
    pub position: (i32, i32),
}

impl Cell {
    pub fn new(all_tiles: &[TileType], position: (i32, i32)) -> Self {
        Self {
            is_collapsed: false,
            tile_type: None,
            entropy: all_tiles.len() as i32,
            valid_tiles: all_tiles.to_vec(),
            position,
        }
    }

    pub fn is_contradiction(&self) -> bool {
        self.valid_tiles.is_empty() && !self.is_collapsed
    }
}

fn update_spatial_index(
    mut spatial_index: ResMut<CellSpatialIndex>,
    added_cells: Query<(Entity, &Cell), Added<Cell>>,
    mut removed_cells: RemovedComponents<Cell>,
) {
    // PSEUDO CODE for update_spatial_index function:

    // 1. Handle newly added cells:
    //    a. Iterate through all entities that have Cell component added this frame
    //    b. For each entity-cell pair:
    //       - Insert mapping from cell's position to entity in spatial index grid
    //       - This allows O(1) lookup of entities by their grid coordinates

    // 2. Handle removed cells:
    //    a. Iterate through all entities that had Cell component removed this frame
    //    b. For each removed entity:
    //       - Remove all grid entries that map to this entity
    //       - Use retain to filter out entries where stored entity matches removed entity
    //       - This prevents dangling references in the spatial index

    for (entity, cell) in added_cells.iter() {
        spatial_index.grid.insert(cell.position, entity);
    }

    for entity in removed_cells.read() {
        spatial_index
            .grid
            .retain(|_, &mut stored_entity| stored_entity != entity);
    }
}

fn initialize_new_cells(
    mut wfc_queue: ResMut<RegularPropagationQueue>,
    added_cells: Query<Entity, Added<Cell>>,
    spatial_index: Res<CellSpatialIndex>,
    cells: Query<&Cell>,
) {
    // PSEUDO CODE for initialize_new_cells function:

    // 1. Iterate through all entities that have Cell component added this frame
    // 2. For each newly added cell entity:
    //    a. Get the cell component data (position, etc.)
    //    b. If cell data retrieval fails, skip to next entity
    //
    //    c. Check all neighboring positions (North, South, East, West):
    //       - Calculate neighbor position using direction vectors
    //       - Look up neighbor entity in spatial index using position
    //       - If neighbor exists:
    //         * Get neighbor's cell data
    //         * If neighbor is collapsed (has a definite tile type):
    //           - Add current entity to propagation queue
    //           - Break out of neighbor checking loop (one collapsed neighbor is enough)
    //
    // Purpose: When a new cell is added to the grid, if it has any collapsed neighbors,
    // it needs to have its valid tiles constrained based on those neighbors' rules.
    // Adding to queue triggers constraint propagation in the next system.
    // If we didn't add them to the propagation queue, those newly added cells would not have their valid tiles updated based on the collapsed neighbors.
    // They would remain with their initial valid tiles, which could lead to inconsistencies in the wave function collapse algorithm.
    for entity in added_cells.iter() {
        if let Ok(cell) = cells.get(entity) {
            for (_, (dx, dz)) in DIRECTION_VECTORS.iter() {
                let neighbor_pos = (cell.position.0 + dx, cell.position.1 + dz);
                if let Some(neighbor_entity) = spatial_index.grid.get(&neighbor_pos) {
                    if let Ok(neighbor_cell) = cells.get(*neighbor_entity) {
                        if neighbor_cell.is_collapsed {
                            wfc_queue.queue.push_back(entity);
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn collapse_lowest_entropy_cell(
    mut wfc_queue: ResMut<RegularPropagationQueue>,
    mut cells: Query<(Entity, &mut Cell)>,
    rules: Res<OpenSpaceRules>,
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

    if !wfc_queue.queue.is_empty() {
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
            let tile = get_random_tile(&rules, &cell.valid_tiles);
            cell.tile_type = Some(tile);
            cell.is_collapsed = true;
            cell.entropy = 0;

            wfc_queue.queue.push_back(entity);
        }
    }
}

// What is the difference between break and return
// break exits the current loop, while return exits the entire function.

fn propagate_building_constraints(
    mut wfc_queue: ResMut<BuildingPropagationQueue>,
    rules: Res<OpenSpaceRules>,
    spatial_index: Res<CellSpatialIndex>,
    mut cells: Query<&mut Cell>,
) {
}

fn propagate_regular_constraints(
    mut wfc_queue: ResMut<RegularPropagationQueue>,
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
                                neighbor_cell.valid_tiles = vec![TileType::Ground];
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
                        cell.valid_tiles = vec![TileType::Ground];
                        cell.entropy = 1;
                    }
                }
            }
        }
    }
}
