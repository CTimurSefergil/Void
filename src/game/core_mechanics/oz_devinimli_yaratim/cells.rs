use bevy::prelude::*;
use std::{collections::HashSet, time::Duration};

use crate::game::core_mechanics::oz_devinimli_yaratim::tiles_meshes_models::{
    CHEST, CORNER, GROUND, WALL,
};
use crate::game::spawn::player::Player;

use super::odycore::Cell;
use super::odyrules::*;
use super::tiles_meshes_models::{TileMaterials, TileMeshes};

const UPDATE_INTERVAL_MS: u64 = 200;
const DESPAWN_INTERVAL_MS: u64 = 1000;

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
            total_cells_on_edge: 19,
            spawn_distance: 0.7,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ODYRules>()
        .init_resource::<GenerationSettings>()
        .add_systems(Update, (create_cells, destroy_cells, update_tile_visuals));
}

#[derive(Component)]
pub struct Tile;

fn create_cells(
    mut commands: Commands,
    player_pos: Single<&Transform, With<Player>>,
    tile_meshes: Res<TileMeshes>,
    tile_materials: Res<TileMaterials>,
    existing_cells: Query<&Transform, With<Cell>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
    wfc_rules: Res<ODYRules>,
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
                Mesh3d(tile_meshes.placeholder.clone()),
                MeshMaterial3d(tile_materials.placeholder.clone()),
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
            commands.entity(entity).despawn();
        }
    }
}

fn update_tile_visuals(
    mut commands: Commands,
    changed_cells: Query<(Entity, &Cell, &Transform), Changed<Cell>>,
    tile_meshes: Res<TileMeshes>,
    tile_materials: Res<TileMaterials>,
) {
    for (entity, cell, transform) in changed_cells.iter() {
        if let Some(tile_type) = cell.tile_type {
            match tile_type {
                TileType::Ground => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        GROUND[1] / 2.0,
                        0.0 + transform.translation.z,
                    ));
                    commands.entity(entity).insert((
                        Mesh3d(tile_meshes.ground.clone()),
                        MeshMaterial3d(tile_materials.ground.clone()),
                        Ground,
                        transform,
                    ));
                }
                TileType::Wall => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        WALL[1] / 2.0,
                        0.0 + transform.translation.z,
                    ));
                    commands.entity(entity).insert((
                        Mesh3d(tile_meshes.wall.clone()),
                        MeshMaterial3d(tile_materials.wall.clone()),
                        Wall,
                        transform,
                    ));
                }
                TileType::Corner => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        CORNER[1] / 2.0,
                        0.0 + transform.translation.z,
                    ));
                    commands.entity(entity).insert((
                        Mesh3d(tile_meshes.corner.clone()),
                        MeshMaterial3d(tile_materials.corner.clone()),
                        Corner,
                        transform,
                    ));
                }
                TileType::Chest => {
                    let transform = Transform::from_translation(Vec3::new(
                        0.0 + transform.translation.x,
                        CHEST[1] / 2.0,
                        0.0 + transform.translation.z,
                    ));
                    commands.entity(entity).insert((
                        Mesh3d(tile_meshes.chest.clone()),
                        MeshMaterial3d(tile_materials.chest.clone()),
                        Chest,
                        transform,
                    ));
                }
            };
        }
    }
}
