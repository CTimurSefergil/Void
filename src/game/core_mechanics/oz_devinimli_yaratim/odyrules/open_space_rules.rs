use bevy::{ecs::resource::Resource, platform::collections::HashMap};
use strum::IntoEnumIterator;

use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::{
    DIRECTIONS, Direction, Rules, TileType,
};

#[derive(Resource, Debug)]
pub struct OpenSpaceRules {
    pub allowed_neighbors: HashMap<TileType, HashMap<Direction, Vec<TileType>>>,
    pub all_tiles: Vec<TileType>,
    pub weights: HashMap<TileType, f32>,
}

impl Rules for OpenSpaceRules {
    fn allowed_neighbors<'a>(&'a self) -> &'a HashMap<TileType, HashMap<Direction, Vec<TileType>>> {
        &self.allowed_neighbors
    }

    fn weights<'a>(&'a self) -> &'a HashMap<TileType, f32> {
        &self.weights
    }
}

impl OpenSpaceRules {
    fn set_all_directions(rules_map: &mut HashMap<Direction, Vec<TileType>>, tiles: Vec<TileType>) {
        for dir in DIRECTIONS.iter() {
            rules_map.insert(*dir, tiles.clone());
        }
    }
}

impl Default for OpenSpaceRules {
    fn default() -> Self {
        let mut allowed_neighbors = HashMap::new();
        let mut rules_map = HashMap::new();

        ////////////////////////////////////////////////////////////////////////////
        // GROUND
        OpenSpaceRules::set_all_directions(
            &mut rules_map,
            vec![
                TileType::Ground,
                TileType::Tree,
                TileType::Chest,
                TileType::FountainCorner1,
                TileType::FountainCorner2,
                TileType::FountainCorner3,
                TileType::FountainCorner4,
                TileType::FountainEdge1,
                TileType::FountainEdge2,
                TileType::FountainEdge3,
                TileType::FountainEdge4,
            ],
        );
        allowed_neighbors.insert(TileType::Ground, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // TREE
        OpenSpaceRules::set_all_directions(&mut rules_map, vec![TileType::Ground, TileType::Tree]);
        allowed_neighbors.insert(TileType::Tree, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // CHEST
        OpenSpaceRules::set_all_directions(&mut rules_map, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Chest, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN CENTER
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge1, TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge4, TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge2, TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge3, TileType::FountainCenter],
        );
        allowed_neighbors.insert(TileType::FountainCenter, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN CORNER1 (Top-left)
        rules_map.insert(Direction::Front, vec![TileType::Ground]);
        rules_map.insert(Direction::Back, vec![TileType::FountainEdge3]);
        rules_map.insert(Direction::Right, vec![TileType::FountainEdge1]);
        rules_map.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::FountainCorner1, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN CORNER2 (Top-right)
        rules_map.insert(Direction::Front, vec![TileType::Ground]);
        rules_map.insert(Direction::Back, vec![TileType::FountainEdge2]);
        rules_map.insert(Direction::Right, vec![TileType::Ground]);
        rules_map.insert(Direction::Left, vec![TileType::FountainEdge1]);
        allowed_neighbors.insert(TileType::FountainCorner2, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN CORNER3 (Bottom-left)
        rules_map.insert(Direction::Front, vec![TileType::FountainEdge3]);
        rules_map.insert(Direction::Back, vec![TileType::Ground]);
        rules_map.insert(Direction::Right, vec![TileType::FountainEdge4]);
        rules_map.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::FountainCorner3, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN CORNER4 (Bottom-right)
        rules_map.insert(Direction::Front, vec![TileType::FountainEdge2]);
        rules_map.insert(Direction::Back, vec![TileType::Ground]);
        rules_map.insert(Direction::Right, vec![TileType::Ground]);
        rules_map.insert(Direction::Left, vec![TileType::FountainEdge4]);
        allowed_neighbors.insert(TileType::FountainCorner4, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN EDGE1 (Top edge - expandable horizontally)
        rules_map.insert(Direction::Front, vec![TileType::Ground]);
        rules_map.insert(Direction::Back, vec![TileType::FountainCenter]);
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge1, TileType::FountainCorner2],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge1, TileType::FountainCorner1],
        );
        allowed_neighbors.insert(TileType::FountainEdge1, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN EDGE2 (Right edge - expandable vertically)
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge2, TileType::FountainCorner2],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge2, TileType::FountainCorner4],
        );
        rules_map.insert(Direction::Right, vec![TileType::Ground]);
        rules_map.insert(Direction::Left, vec![TileType::FountainCenter]);
        allowed_neighbors.insert(TileType::FountainEdge2, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN EDGE3 (Left edge - expandable vertically)
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge3, TileType::FountainCorner1],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge3, TileType::FountainCorner3],
        );
        rules_map.insert(Direction::Right, vec![TileType::FountainCenter]);
        rules_map.insert(Direction::Left, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::FountainEdge3, rules_map.clone());
        rules_map.clear();

        ////////////////////////////////////////////////////////////////////////////
        // FOUNTAIN EDGE4 (Bottom edge - expandable horizontally)
        rules_map.insert(Direction::Front, vec![TileType::FountainCenter]);
        rules_map.insert(Direction::Back, vec![TileType::Ground]);
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge4, TileType::FountainCorner4],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge4, TileType::FountainCorner3],
        );
        allowed_neighbors.insert(TileType::FountainEdge4, rules_map.clone());
        rules_map.clear();

        let mut weights = HashMap::new();
        for tile in TileType::iter() {
            let weight = match tile {
                TileType::Ground => 0.2,
                TileType::Tree => 0.1,
                TileType::Chest => 0.01,
                TileType::FountainCenter => 0.345678,
                TileType::FountainCorner1 => 0.34567,
                TileType::FountainCorner2 => 0.3456,
                TileType::FountainCorner3 => 0.345,
                TileType::FountainCorner4 => 0.34,
                TileType::FountainEdge1 => 0.339,
                TileType::FountainEdge2 => 0.338,
                TileType::FountainEdge3 => 0.337,
                TileType::FountainEdge4 => 0.336,
                TileType::Empty => 0.00001,
            };
            weights.insert(tile, weight);
        }

        OpenSpaceRules {
            allowed_neighbors,
            all_tiles: vec![
                TileType::Ground,
                TileType::Tree,
                TileType::Chest,
                TileType::FountainCenter,
                TileType::FountainCorner1,
                TileType::FountainCorner2,
                TileType::FountainCorner3,
                TileType::FountainCorner4,
                TileType::FountainEdge1,
                TileType::FountainEdge2,
                TileType::FountainEdge3,
                TileType::FountainEdge4,
            ],
            weights,
        }
    }
}
