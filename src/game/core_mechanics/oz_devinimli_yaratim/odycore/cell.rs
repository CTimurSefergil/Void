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
    added_cells: Query<(Entity, &Cell), Added<Cell>>,
    spatial_index: Res<CellSpatialIndex>,
    cells: Query<&Cell>,
) {
    for (entity, cell) in added_cells.iter() {
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
