use bevy::{
    ecs::system::entity_command::insert_from_world, platform::collections::HashSet, prelude::*,
};
use std::collections::HashMap;

use super::algoritma::Tile;

pub(super) fn plugin(app: &mut App) {}

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

#[derive(Resource)]
pub struct WFCRules {
    pub allowed_neighbors: HashMap<TileType, HashMap<Direction, Vec<TileType>>>,
}

impl Default for WFCRules {
    fn default() -> Self {
        let mut allowed_neighbors = HashMap::new();
        let mut direc_vec_tile = HashMap::new();

        // GROUND
        direc_vec_tile.clear();
        direc_vec_tile.insert(Direction::Front, vec![TileType::Wall]);
        direc_vec_tile.insert(Direction::Back, vec![TileType::Wall]);
        direc_vec_tile.insert(Direction::Right, vec![TileType::Wall]);
        direc_vec_tile.insert(Direction::Left, vec![TileType::Wall]);
        allowed_neighbors.insert(TileType::Ground, direc_vec_tile.clone());
        // WALL
        direc_vec_tile.clear();
        direc_vec_tile.insert(Direction::Front, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Back, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Right, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Wall, direc_vec_tile.clone());
        // TREE
        direc_vec_tile.clear();
        direc_vec_tile.insert(Direction::Front, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Back, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Right, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Tree, direc_vec_tile.clone());
        // COLUMN
        direc_vec_tile.clear();
        direc_vec_tile.insert(Direction::Front, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Back, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Right, vec![TileType::Ground]);
        direc_vec_tile.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Column, direc_vec_tile.clone());

        WFCRules { allowed_neighbors }
    }
}

#[derive(Component)]
pub struct Cell {
    pub position: (f32, f32),
    pub allowed_neighbors: HashSet<TileType>,
    pub is_collapsed: bool,
    pub tile_type: Option<TileType>,
}

impl Cell {}
