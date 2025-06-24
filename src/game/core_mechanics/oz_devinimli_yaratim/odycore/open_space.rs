use bevy::{platform::collections::HashMap, prelude::*};
use rand::seq::IteratorRandom;

use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::{
    commons::{DIRECTION_VECTORS, Direction, TileType},
    open_space_rules::OpenSpaceRules,
};

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

#[derive(Resource)]
pub struct SpatialIndex {
    cells: HashMap<(i32, i32), Entity>,
}

pub fn update_spatial_index(
    cells: Query<(Entity, &Transform), Added<Cell>>,
    mut removed_cells: RemovedComponents<Cell>,
    mut spatial_index: ResMut<SpatialIndex>,
) {
    for (entity, pos) in cells.iter() {
        spatial_index
            .cells
            .insert((pos.translation.x as i32, pos.translation.z as i32), entity);
    }
    for entity in removed_cells.read() {
        spatial_index
            .cells
            .retain(|_, &mut removed_entity| removed_entity != entity);
    }
}

pub fn collapse_lowest_entropy_cell(mut cells: Query<&mut Cell>) {
    //  Hücrelerin olası seçenek sayısını al
    let mut candidates = cells
        .iter_mut()
        .filter(|cell| !cell.is_collapsed)
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        return;
    }

    let lowest_entropy = candidates.iter().map(|cell| cell.entropy).min().unwrap();
    candidates.retain(|cell| cell.entropy == lowest_entropy);

<<<<<<< Updated upstream
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
=======
    //  Birini seç ve tipini belirle
    let mut rng = rand::rng();
    if let Some(mut chosen_cell) = candidates.into_iter().choose(&mut rng) {
        if !chosen_cell.valid_tiles.is_empty() {
            if let Some(&tile) = chosen_cell.valid_tiles.iter().choose(&mut rng) {
                chosen_cell.tile_type = Some(tile);
                chosen_cell.is_collapsed = true;
            }
>>>>>>> Stashed changes
        }
    }
}

pub fn propagate_newly_added_cells(
    mut cells: Query<(&mut Cell, &Transform), Added<Cell>>,
    spatial_index: Res<SpatialIndex>,
    rules: Res<OpenSpaceRules>,
) {
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

        for (direction, (x, z)) in DIRECTION_VECTORS {
            if let Some(neighbor_entity) = spatial_index.cells.get(&(
                transform.translation.x as i32 + x,
                transform.translation.z as i32 + z,
            )) {
                if let Ok((neighbor_cell, _)) = cells_query.get_mut(*neighbor_entity) {
                    if let Some(tile_type_of_neighbor) = neighbor_cell.tile_type {
                        let current_valid_tiles = filter_valid_tiles(
                            tile_type_of_neighbor,
                            cell.valid_tiles.clone(),
                            direction,
                            rules.as_ref(),
                        );

<<<<<<< Updated upstream
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
=======
                        let entropy = current_valid_tiles.len() as i32;
                        cell.valid_tiles = current_valid_tiles;
                        cell.entropy = entropy;
>>>>>>> Stashed changes
                    }
                }
            }
        }
    
}

pub fn filter_valid_tiles(
    tile_type_of_neighbor: TileType,
    valid_tiles: Vec<TileType>,
    direction: Direction,
    rules: &OpenSpaceRules,
) -> Vec<TileType> {
    let mut new_valid_tiles = Vec::new();
    if let Some(neighbors_valid_tiles) = rules.allowed_neighbors.get(&tile_type_of_neighbor) {
        for tile in valid_tiles.iter() {
            if neighbors_valid_tiles
                .get(&direction)
                .unwrap()
                .contains(tile)
            {
                new_valid_tiles.push(*tile);
            }
        }
    }

    new_valid_tiles
}
