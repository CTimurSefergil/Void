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

    pub fn is_contradicted(&mut self) -> bool {
        self.valid_tiles.is_empty()
    }
}

pub fn update_spatial_index(
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
