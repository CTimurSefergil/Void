use bevy::prelude::*;
use std::collections::HashMap;

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

        let mut ground_rules = HashMap::new();
        ground_rules.insert(
            Direction::Front,
            vec![
                TileType::Wall,
                TileType::Tree,
                TileType::Column,
                TileType::Ground,
            ],
        );
        ground_rules.insert(
            Direction::Back,
            vec![
                TileType::Wall,
                TileType::Tree,
                TileType::Column,
                TileType::Ground,
            ],
        );
        ground_rules.insert(
            Direction::Left,
            vec![
                TileType::Wall,
                TileType::Tree,
                TileType::Column,
                TileType::Ground,
            ],
        );
        ground_rules.insert(
            Direction::Right,
            vec![
                TileType::Wall,
                TileType::Tree,
                TileType::Column,
                TileType::Ground,
            ],
        );
        allowed_neighbors.insert(TileType::Ground, ground_rules);

        let mut wall_rules = HashMap::new();
        wall_rules.insert(Direction::Front, vec![TileType::Wall, TileType::Ground]);
        wall_rules.insert(Direction::Back, vec![TileType::Wall, TileType::Ground]);
        wall_rules.insert(Direction::Left, vec![TileType::Wall, TileType::Ground]);
        wall_rules.insert(Direction::Right, vec![TileType::Wall, TileType::Ground]);
        allowed_neighbors.insert(TileType::Wall, wall_rules);

        let mut tree_rules = HashMap::new();
        tree_rules.insert(Direction::Front, vec![TileType::Tree, TileType::Ground]);
        tree_rules.insert(Direction::Back, vec![TileType::Tree, TileType::Ground]);
        tree_rules.insert(Direction::Left, vec![TileType::Tree, TileType::Ground]);
        tree_rules.insert(Direction::Right, vec![TileType::Tree, TileType::Ground]);
        allowed_neighbors.insert(TileType::Tree, tree_rules);

        let mut column_rules = HashMap::new();
        column_rules.insert(Direction::Front, vec![TileType::Column, TileType::Ground]);
        column_rules.insert(Direction::Back, vec![TileType::Column, TileType::Ground]);
        column_rules.insert(Direction::Left, vec![TileType::Column, TileType::Ground]);
        column_rules.insert(Direction::Right, vec![TileType::Column, TileType::Ground]);
        allowed_neighbors.insert(TileType::Column, column_rules);

        Self { allowed_neighbors }
    }
}

#[derive(Resource)]
pub struct AssetData {
    pub assets: HashMap<TileType, String>,
}

impl Default for AssetData {
    fn default() -> Self {
        let mut assets = HashMap::new();
        assets.insert(TileType::Wall, "models/wall".to_string());
        assets.insert(TileType::Tree, "models/tree".to_string());
        assets.insert(TileType::Column, "models/column".to_string());
        assets.insert(TileType::Ground, "models/floor".to_string());
        Self { assets }
    }
}
