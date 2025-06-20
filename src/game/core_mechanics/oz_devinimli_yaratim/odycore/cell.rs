use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::Added,
        removal_detection::RemovedComponents,
        resource::Resource,
        system::{Query, Res, ResMut},
    },
    platform::collections::HashMap,
};

use crate::game::core_mechanics::oz_devinimli_yaratim::{
    odycore::open_space::OpenSpacePropagationQueue,
    odyrules::commons::{DIRECTION_VECTORS, TileType},
};

#[derive(Resource, Default)]
pub struct CellSpatialIndex {
    pub grid: HashMap<(i32, i32), Entity>,
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

pub fn update_spatial_index(
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

pub fn initialize_new_cells(
    mut wfc_queue: ResMut<OpenSpacePropagationQueue>,
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
