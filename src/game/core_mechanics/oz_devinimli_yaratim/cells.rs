use bevy::{platform::collections::HashMap, prelude::*};
use std::{collections::HashSet, time::Duration};

use crate::game::{
    core_mechanics::oz_devinimli_yaratim::odyrules::{
        commons::TileType, open_space_rules::OpenSpaceRules,
    },
    spawn::player::Player,
};
const UPDATE_INTERVAL_MS: u64 = 200;
const DESPAWN_INTERVAL_MS: u64 = 200;

#[derive(Resource)]
pub struct GenerationSettings {
    pub cell_edge_length: i32,
    pub total_cells_on_edge: i32,
    pub spawn_distance: f32,
}

impl Default for GenerationSettings {
    fn default() -> Self {
        Self {
            cell_edge_length: 9,
            total_cells_on_edge: 17,
            spawn_distance: 0.7,
        }
    }
}

#[derive(Resource, Default)]
pub struct CellSpatialIndex {
    pub grid: HashMap<(i32, i32), Entity>,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GenerationSettings>()
        .init_resource::<CellSpatialIndex>()
        .add_systems(
            Update,
            (create_cells, update_spatial_index, destroy_cells).chain(),
        );
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
) {
    for (entity, cell) in added_cells.iter() {
        spatial_index.grid.insert(cell.position, entity);
    }
}

#[derive(Component)]
pub struct Tile;

fn create_cells(
    mut commands: Commands,
    player_pos: Single<&Transform, With<Player>>,
    existing_cells: Query<&Transform, With<Cell>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
    wfc_rules: Res<OpenSpaceRules>,
    settings: Res<GenerationSettings>,
) {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(UPDATE_INTERVAL_MS) > now {
        return;
    }
    *last_update = now;

    let player_grid_x =
        (player_pos.translation.x / settings.cell_edge_length as f32).round() as i32;
    let player_grid_z =
        (player_pos.translation.z / settings.cell_edge_length as f32).round() as i32;

    let existing_positions: HashSet<(i32, i32)> = existing_cells
        .iter()
        .map(|transform| {
            let grid_x =
                (transform.translation.x / settings.cell_edge_length as f32).round() as i32;
            let grid_z =
                (transform.translation.z / settings.cell_edge_length as f32).round() as i32;
            (grid_x, grid_z)
        })
        .collect();

    let half_size = settings.total_cells_on_edge / 2;

    for grid_x in (player_grid_x - half_size)..=(player_grid_x + half_size) {
        for grid_z in (player_grid_z - half_size)..=(player_grid_z + half_size) {
            if existing_positions.contains(&(grid_x, grid_z)) {
                continue;
            }

            let world_x = grid_x as f32 * settings.cell_edge_length as f32;
            let world_z = grid_z as f32 * settings.cell_edge_length as f32;
            let position = (grid_x, grid_z);

            let cell = Cell::new(&wfc_rules.all_tiles, position);

            commands.spawn((
                Name::new(format!("Cell_{}_{}", grid_x, grid_z)),
                cell,
                Transform::from_translation(Vec3::new(world_x, 0.0, world_z)),
                Tile,
            ));
        }
    }
}

fn destroy_cells(
    mut commands: Commands,
    player_pos: Single<&Transform, With<Player>>,
    cells: Query<(Entity, &Transform), With<Cell>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
    settings: Res<GenerationSettings>,
    mut spatial_index: ResMut<CellSpatialIndex>,
) {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(DESPAWN_INTERVAL_MS) > now {
        return;
    }
    *last_update = now;

    let despawn_distance = (settings.total_cells_on_edge as f32 * settings.cell_edge_length as f32)
        * settings.spawn_distance;

    for (entity, transform) in cells.iter() {
        if player_pos.translation.distance(transform.translation) > despawn_distance {
            spatial_index.grid.remove(&(
                transform.translation.x as i32,
                transform.translation.z as i32,
            ));
            commands.entity(entity).despawn();
        }
    }
}
