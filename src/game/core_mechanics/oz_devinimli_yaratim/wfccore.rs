use bevy::{platform::collections::HashSet, prelude::*};
use rand::seq::IteratorRandom;
use std::collections::{HashMap, VecDeque};

use super::wfcrules::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<WFCRules>()
        .init_resource::<CellSpatialIndex>()
        .init_resource::<PropagationQueue>()
        .add_systems(Startup, setup_wfc_rules)
        .add_systems(
            Update,
            (
                update_spatial_index,
                initialize_new_cells,
                propagate_constraints,
                collapse_lowest_entropy_cell,
            )
                .chain(),
        );
}

fn setup_wfc_rules(mut commands: Commands) {
    commands.insert_resource(WFCRules::default());
    commands.insert_resource(CellSpatialIndex::default());
    commands.insert_resource(PropagationQueue::default());
}

#[derive(Resource, Default)]
pub struct CellSpatialIndex {
    pub grid: HashMap<(i32, i32), Entity>,
}

#[derive(Resource, Default)]
pub struct PropagationQueue {
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

fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Front => Direction::Back,
        Direction::Back => Direction::Front,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
    }
}

fn filter_valid_tiles(
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

fn update_spatial_index(
    mut spatial_index: ResMut<CellSpatialIndex>,
    added_cells: Query<(Entity, &Cell), Added<Cell>>,
    mut removed_cells: RemovedComponents<Cell>,
) {
    for (entity, cell) in added_cells.iter() {
        spatial_index.grid.insert(cell.position, entity);
    }

    for entity in removed_cells.read() {
        spatial_index
            .grid
            .retain(|_, &mut stored_entity| stored_entity != entity);
    }
}

fn propagate_constraints(
    mut wfc_queue: ResMut<PropagationQueue>,
    rules: Res<WFCRules>,
    spatial_index: Res<CellSpatialIndex>,
    mut cells: Query<&mut Cell>,
) {
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

        // CASE 1: Cell is collapsed - propagate to neighbors
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
                                &rules,
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
        }
        // CASE 2: Cell is new - initialize from neighbors
        else {
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
                                    &rules,
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

fn collapse_lowest_entropy_cell(
    mut wfc_queue: ResMut<PropagationQueue>,
    mut cells: Query<(Entity, &mut Cell)>,
    rules: Res<WFCRules>,
) {
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

fn initialize_new_cells(
    mut wfc_queue: ResMut<PropagationQueue>,
    added_cells: Query<Entity, Added<Cell>>,
    spatial_index: Res<CellSpatialIndex>,
    cells: Query<&Cell>,
) {
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

fn get_random_tile(rules: &WFCRules, valid_tiles: &[TileType]) -> TileType {
    use rand::prelude::*;
    
    if valid_tiles.is_empty() {
        return TileType::Ground;
    }

    let mut rng = rand::rng();
    let total_weight: f32 = valid_tiles.iter()
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

    valid_tiles[0] // Fallback
}