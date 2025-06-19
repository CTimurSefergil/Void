use bevy::{ecs::resource::Resource, platform::collections::HashMap};

use crate::game::core_mechanics::oz_devinimli_yaratim::{cells::Tile, odyrules::commons::{
    Direction, Rules, TileType,
}};

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

impl Default for OpenSpaceRules {
    fn default() -> Self {
        let mut allowed_neighbors = HashMap::new();
        let mut rules_map = HashMap::new();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //GROUND
        rules_map.insert(
            Direction::Front,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        rules_map.insert(
            Direction::Back,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        rules_map.insert(
            Direction::Right,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        rules_map.insert(
            Direction::Left,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        rules_map.insert(
            Direction::FrontRight,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        rules_map.insert(
            Direction::FrontLeft,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        rules_map.insert(
            Direction::BackRight,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        rules_map.insert(
            Direction::BackLeft,
            vec![
                TileType::Ground,
                TileType::Chest,
                TileType::Tree,
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
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //TREE
        rules_map.insert(Direction::Front, vec![TileType::Ground, TileType::Tree]);
        rules_map.insert(Direction::Back, vec![TileType::Ground, TileType::Tree]);
        rules_map.insert(Direction::Right, vec![TileType::Ground, TileType::Tree]);
        rules_map.insert(Direction::Left, vec![TileType::Ground, TileType::Tree]);
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::Ground, TileType::Tree],
        );
        rules_map.insert(Direction::FrontLeft, vec![TileType::Ground, TileType::Tree]);
        rules_map.insert(Direction::BackRight, vec![TileType::Ground, TileType::Tree]);
        rules_map.insert(Direction::BackLeft, vec![TileType::Ground, TileType::Tree]);
        allowed_neighbors.insert(TileType::Tree, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //CHEST
        rules_map.insert(Direction::Front, vec![TileType::Ground]);
        rules_map.insert(Direction::Back, vec![TileType::Ground]);
        rules_map.insert(Direction::Right, vec![TileType::Ground]);
        rules_map.insert(Direction::Left, vec![TileType::Ground]);
        rules_map.insert(Direction::FrontRight, vec![TileType::Ground]);
        rules_map.insert(Direction::FrontLeft, vec![TileType::Ground]);
        rules_map.insert(Direction::BackRight, vec![TileType::Ground]);
        rules_map.insert(Direction::BackLeft, vec![TileType::Ground]);
        allowed_neighbors.insert(TileType::Chest, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN CENTER
        rules_map.insert(
            Direction::Front,
            vec![
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
        );
        rules_map.insert(
            Direction::Back,
            vec![
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
        );
        rules_map.insert(
            Direction::Right,
            vec![
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
        );
        rules_map.insert(
            Direction::Left,
            vec![
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
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![
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
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![
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
        );
        rules_map.insert(
            Direction::BackRight,
            vec![
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
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![
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
        );
        allowed_neighbors.insert(TileType::FountainCenter, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN CORNER1
        rules_map.insert(
            Direction::Front,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge3],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge1],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::Ground],
        );
        allowed_neighbors.insert(TileType::FountainCorner1, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN CORNER2
        rules_map.insert(
            Direction::Front,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge2],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge1],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::FountainCenter],
        );
        allowed_neighbors.insert(TileType::FountainCorner2, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN CORNER3
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge3],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge4],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::Ground],
        );
        allowed_neighbors.insert(TileType::FountainCorner3, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN CORNER4
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge2],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge4],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::Ground],
        );
        allowed_neighbors.insert(TileType::FountainCorner4, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN EDGE1
        rules_map.insert(
            Direction::Front,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge1, TileType::FountainCorner2],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge1, TileType::FountainCorner1],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::FountainEdge2, TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::FountainEdge3, TileType::FountainCenter],
        );
        allowed_neighbors.insert(TileType::FountainEdge1, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN EDGE2
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainEdge2, TileType::FountainCorner2],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainEdge2, TileType::FountainCorner2],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::FountainCenter, TileType::FountainEdge1],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::FountainCenter, TileType::FountainEdge4],
        );
        allowed_neighbors.insert(TileType::FountainEdge2, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN EDGE3
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainCorner1, TileType::FountainEdge3],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::FountainCorner3, TileType::FountainEdge3],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::FountainEdge1, TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::FountainEdge4, TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::Ground],
        );
        allowed_neighbors.insert(TileType::FountainEdge3, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        //FOUNTAIN EDGE4
        rules_map.insert(
            Direction::Front,
            vec![TileType::FountainCenter],
        );
        rules_map.insert(
            Direction::Back,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::Right,
            vec![TileType::FountainEdge4, TileType::FountainCorner4],
        );
        rules_map.insert(
            Direction::Left,
            vec![TileType::FountainEdge4, TileType::FountainCorner3],
        );
        rules_map.insert(
            Direction::FrontRight,
            vec![TileType::FountainCenter, TileType::FountainEdge2],
        );
        rules_map.insert(
            Direction::FrontLeft,
            vec![TileType::FountainCenter, TileType::FountainEdge3],
        );
        rules_map.insert(
            Direction::BackRight,
            vec![TileType::Ground],
        );
        rules_map.insert(
            Direction::BackLeft,
            vec![TileType::Ground],
        );
        allowed_neighbors.insert(TileType::FountainEdge4, rules_map.clone());
        rules_map.clear();
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

        let mut weights = HashMap::new();
        weights.insert(TileType::Ground, 0.8);
        weights.insert(TileType::Tree, 0.2);
        weights.insert(TileType::Chest, 0.01);
        weights.insert(TileType::FountainCenter, 0.3);
        weights.insert(TileType::FountainCorner1, 0.3);
        weights.insert(TileType::FountainCorner2, 0.3);
        weights.insert(TileType::FountainCorner3, 0.3);
        weights.insert(TileType::FountainCorner4, 0.3);
        weights.insert(TileType::FountainEdge1, 0.3);
        weights.insert(TileType::FountainEdge2, 0.3);
        weights.insert(TileType::FountainEdge3, 0.3);
        weights.insert(TileType::FountainEdge4, 0.3);

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
