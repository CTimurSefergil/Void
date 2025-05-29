use std::time::Duration;

use bevy::prelude::*;

use crate::game::spawn::player::Player;

const CELL_EDGE_LENGTH: i32 = 5;
const TOTAL_CELLS_ON_EDGE: i32 = 11;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (create_cells, destroy_cells));
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Wall,
    Tree,
    Column,
    Ground,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    Front, // +Z
    Back,  // -Z
    Right, // +X
    Left,  // -X
}

#[derive(Component)]
pub struct Cell {
    position: (f32, f32),
    allowed_neighbors: Vec<TileType>,
    is_collapsed: bool,
}

#[derive(Component)]
pub struct Tile;

impl Cell {
    fn new(x: f32, z: f32, allowed_neighbors: Vec<TileType>, command: &mut Commands) {
        command.spawn(Self {
            position: (x, z),
            allowed_neighbors: vec![
                TileType::Column,
                TileType::Ground,
                TileType::Tree,
                TileType::Wall,
            ],
            is_collapsed: false,
        });
    }

    fn destroy(entity: Entity, command: &mut Commands) {
        command.entity(entity).despawn();
    }
}

fn create_cells(
    mut command: Commands,
    player_pos: Single<&Transform, With<Player>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
) {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(200) > now {
        return;
    }
    *last_update = now;

    let ball_mesh = mesh_assets.add(Sphere::new(1.0));
    let color = Color::WHITE;
    let ball_material = material_assets.add(StandardMaterial {
        base_color: color,
        ..Default::default()
    });

    for x in 0..TOTAL_CELLS_ON_EDGE {
        for z in 0..TOTAL_CELLS_ON_EDGE {
            let (x, z) = grid_to_world(x, z);
            let (x, z) = (
                (((player_pos.translation.x
                    + ((TOTAL_CELLS_ON_EDGE as f32 - 1.0) / 2.0) * CELL_EDGE_LENGTH as f32)
                    - x)
                    / CELL_EDGE_LENGTH as f32)
                    .round()
                    * CELL_EDGE_LENGTH as f32,
                (((player_pos.translation.z
                    + ((TOTAL_CELLS_ON_EDGE as f32 - 1.0) / 2.0) * CELL_EDGE_LENGTH as f32)
                    - z)
                    / CELL_EDGE_LENGTH as f32)
                    .round()
                    * CELL_EDGE_LENGTH as f32,
            );

            Cell::new(x, z, vec![TileType::Ground], &mut command);
            command.spawn((
                Transform::from_translation(Vec3::new(x, 0.0, z)),
                Mesh3d(ball_mesh.clone()),
                MeshMaterial3d(ball_material.clone()),
                Tile,
            ));
        }
    }
}

fn destroy_cells(
    mut command: Commands,
    player_pos: Single<&Transform, With<Player>>,
    cells: Query<(Entity, &Transform), With<Cell>>,
    tiles: Query<(Entity, &Transform), With<Tile>>,
    mut last_update: Local<Duration>,
    time: Res<Time>,
) {
    let now = time.elapsed();
    if *last_update + Duration::from_millis(1000) > now {
        return;
    }
    *last_update = now;

    for (cell, pos) in cells.iter() {
        if player_pos.translation.distance(pos.translation) > 40.0 {
            command.entity(cell).despawn();
        }
    }

    for (tile, pos) in tiles.iter() {
        if player_pos.translation.distance(pos.translation) > 40.0 {
            command.entity(tile).despawn();
        }
    }
}

fn grid_to_world(x: i32, z: i32) -> (f32, f32) {
    ((x * CELL_EDGE_LENGTH) as f32, (z * CELL_EDGE_LENGTH) as f32)
}
