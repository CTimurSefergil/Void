use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::Added,
        removal_detection::RemovedComponents,
        resource::Resource,
        system::{Query, ResMut},
    },
    platform::collections::HashMap,
};

use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::TileType;

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

    pub fn update_entropy(&mut self) {
        if !self.is_collapsed {
            self.entropy = self.valid_tiles.len() as i32;
        }
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
