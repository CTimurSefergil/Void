use bevy::{ecs::resource::Resource, platform::collections::HashMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Ground,
    Wall,
    Corner,
    Chest,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    Front,
    Back,
    Right,
    Left,
}

const _DIRECTIONS: [Direction; 4] = [
    Direction::Front,
    Direction::Back,
    Direction::Right,
    Direction::Left,
];

pub const DIRECTION_VECTORS: [(Direction, (i32, i32)); 4] = [
    (Direction::Front, (0, 1)),
    (Direction::Back, (0, -1)),
    (Direction::Right, (1, 0)),
    (Direction::Left, (-1, 0)),
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

        rules_map.insert(Direction::Front, vec![TileType::Ground, TileType::Chest, TileType::Corner]);
        rules_map.insert(Direction::Back, vec![TileType::Ground, TileType::Chest, TileType::Corner]);
        rules_map.insert(Direction::Right, vec![TileType::Ground, TileType::Chest, TileType::Corner]);
        rules_map.insert(Direction::Left, vec![TileType::Ground, TileType::Chest, TileType::Corner]);
        allowed_neighbors.insert(TileType::Ground, rules_map.clone());

        rules_map.clear();
        rules_map.insert(Direction::Front, vec![TileType::Ground]);
        rules_map.insert(Direction::Back, vec![TileType::Ground]);
        rules_map.insert(Direction::Right, vec![TileType::Ground]);
        rules_map.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Wall, rules_map.clone());
        
        rules_map.clear();
        rules_map.insert(Direction::Front, vec![TileType::Ground, TileType::Corner]);
        rules_map.insert(Direction::Back, vec![TileType::Ground, TileType::Corner]);
        rules_map.insert(Direction::Right, vec![TileType::Ground, TileType::Corner]);
        rules_map.insert(Direction::Left, vec![TileType::Ground, TileType::Corner]);
        allowed_neighbors.insert(TileType::Corner, rules_map.clone());

        rules_map.clear();
        rules_map.insert(Direction::Front, vec![TileType::Ground]);
        rules_map.insert(Direction::Back, vec![TileType::Ground]);
        rules_map.insert(Direction::Right, vec![TileType::Ground]);
        rules_map.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Chest, rules_map.clone());

        let mut weights = HashMap::new();
        weights.insert(TileType::Ground, 0.8);
        weights.insert(TileType::Wall, 0.0);
        weights.insert(TileType::Corner, 0.2);
        weights.insert(TileType::Chest, 0.01);

        ODYRules {
            allowed_neighbors,
            all_tiles: vec![
                TileType::Ground,
                TileType::Wall,
                TileType::Corner,
                TileType::Chest,
            ],
            weights,
        }
    }
}
