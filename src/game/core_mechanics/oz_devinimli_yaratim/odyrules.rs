use bevy::{
    ecs::{component::Component, resource::Resource},
    platform::collections::HashMap,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Ground,
    Tree,
    Chest,
}
#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Tree;
#[derive(Component)]
pub struct Chest;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    ZeroZeroOne,
    ZeroZeroMinusOne,
    OneZeroZero,
    MinusOneZeroZero,
}

const _DIRECTIONS: [Direction; 4] = [
    Direction::ZeroZeroOne,
    Direction::ZeroZeroMinusOne,
    Direction::OneZeroZero,
    Direction::MinusOneZeroZero,
];

pub const DIRECTION_VECTORS: [(Direction, (i32, i32)); 4] = [
    (Direction::ZeroZeroOne, (0, 1)),
    (Direction::ZeroZeroMinusOne, (0, -1)),
    (Direction::OneZeroZero, (1, 0)),
    (Direction::MinusOneZeroZero, (-1, 0)),
];

#[derive(Resource, Debug)]
pub struct ODYRules {
    pub allowed_neighbors: HashMap<TileType, HashMap<Direction, Vec<TileType>>>,
    pub all_tiles: Vec<TileType>,
    pub weights: HashMap<TileType, f32>,
}

impl Default for ODYRules {
    fn default() -> Self {
        let mut allowed_neighbors = HashMap::new();
        let mut rules_map = HashMap::new();

        rules_map.insert(
            Direction::ZeroZeroOne,
            vec![TileType::Ground, TileType::Chest, TileType::Tree],
        );
        rules_map.insert(
            Direction::ZeroZeroMinusOne,
            vec![TileType::Ground, TileType::Chest, TileType::Tree],
        );
        rules_map.insert(
            Direction::OneZeroZero,
            vec![TileType::Ground, TileType::Chest, TileType::Tree],
        );
        rules_map.insert(
            Direction::MinusOneZeroZero,
            vec![TileType::Ground, TileType::Chest, TileType::Tree],
        );
        allowed_neighbors.insert(TileType::Ground, rules_map.clone());

        rules_map.clear();
        rules_map.insert(
            Direction::ZeroZeroOne,
            vec![TileType::Ground, TileType::Tree],
        );
        rules_map.insert(
            Direction::ZeroZeroMinusOne,
            vec![TileType::Ground, TileType::Tree],
        );
        rules_map.insert(
            Direction::OneZeroZero,
            vec![TileType::Ground, TileType::Tree],
        );
        rules_map.insert(
            Direction::MinusOneZeroZero,
            vec![TileType::Ground, TileType::Tree],
        );
        allowed_neighbors.insert(TileType::Tree, rules_map.clone());

        rules_map.clear();
        rules_map.insert(Direction::ZeroZeroOne, vec![TileType::Ground]);
        rules_map.insert(Direction::ZeroZeroMinusOne, vec![TileType::Ground]);
        rules_map.insert(Direction::OneZeroZero, vec![TileType::Ground]);
        rules_map.insert(Direction::MinusOneZeroZero, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Chest, rules_map.clone());

        let mut weights = HashMap::new();
        weights.insert(TileType::Ground, 0.8);
        weights.insert(TileType::Tree, 0.2);
        weights.insert(TileType::Chest, 0.01);

        ODYRules {
            allowed_neighbors,
            all_tiles: vec![TileType::Ground, TileType::Tree, TileType::Chest],
            weights,
        }
    }
}
