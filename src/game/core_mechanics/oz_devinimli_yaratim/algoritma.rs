use bevy::platform::collections::HashSet;
use rand::prelude::*;
use std::time::Duration;

use bevy::prelude::*;

use crate::game::spawn::player::Player;

use super::tiles_meshes_models::TileMaterials;
use super::tiles_meshes_models::TileMeshes;
use super::wfccore::Cell;
use super::wfccore::TileType;

const CELL_EDGE_LENGTH: i32 = 5;
const TOTAL_CELLS_ON_EDGE: i32 = 17;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (create_cells, destroy_cells));
}

#[derive(Component)]
pub struct Tile;

fn create_cells(
    mut command: Commands,
    player_pos: Single<&Transform, With<Player>>,
    tile_meshes: Res<TileMeshes>,
    tile_materials: Res<TileMaterials>,
    existing_cells: Query<&Cell>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
) {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(200) > now {
        return;
    }
    *last_update = now;

    let player_grid_x = (player_pos.translation.x / CELL_EDGE_LENGTH as f32).round() as i32;
    let player_grid_z = (player_pos.translation.z / CELL_EDGE_LENGTH as f32).round() as i32;

    // BURASI ÇOK TAŞAKLI
    let existing_positions: HashSet<(i32, i32)> = existing_cells
        .iter()
        .map(|cell| {
            let grid_x = (cell.position.0 / CELL_EDGE_LENGTH as f32).round() as i32;
            let grid_z = (cell.position.1 / CELL_EDGE_LENGTH as f32).round() as i32;
            (grid_x, grid_z)
        })
        .collect();

    let half_size = TOTAL_CELLS_ON_EDGE / 2;

    for grid_x in (player_grid_x - half_size)..(player_grid_x + half_size) {
        for grid_z in (player_grid_z - half_size)..(player_grid_z + half_size) {
            if !existing_positions.contains(&(grid_x, grid_z)) {
                let world_x = grid_x as f32 * CELL_EDGE_LENGTH as f32;
                let world_z = grid_z as f32 * CELL_EDGE_LENGTH as f32;

                command.spawn((
                    Cell {
                        position: (world_x, world_z),
                        allowed_neighbors: HashSet::from([TileType::Ground]),
                        is_collapsed: false,
                        tile_type: None,
                    },
                    Transform::from_translation(Vec3::new(world_x, 0.0, world_z)),
                    Mesh3d(tile_meshes.sphere.clone()),
                    MeshMaterial3d(tile_materials.white.clone()),
                    Tile,
                ));
            }
        }
    }
}

fn destroy_cells(
    mut command: Commands,
    player_pos: Single<&Transform, With<Player>>,
    cells: Query<(Entity, &Transform), With<Cell>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
) {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(1000) > now {
        return;
    }
    *last_update = now;

    let despawn_distance = (TOTAL_CELLS_ON_EDGE as f32 * CELL_EDGE_LENGTH as f32) * 0.75;

    for (entity, transform) in cells.iter() {
        if player_pos.translation.distance(transform.translation) > despawn_distance {
            command.entity(entity).despawn();
        }
    }
}
