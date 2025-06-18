use bevy::{ecs::resource::Resource, platform::collections::HashMap};

use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::{
    Direction, Rules, TileType,
};

#[derive(Resource, Debug)]
pub struct BuildingRules {
    pub allowed_neighbors: HashMap<TileType, HashMap<Direction, Vec<TileType>>>,
    pub all_tiles: Vec<TileType>,
    pub weights: HashMap<TileType, f32>,
}

impl Rules for BuildingRules {
    fn allowed_neighbors<'a>(&'a self) -> &'a HashMap<TileType, HashMap<Direction, Vec<TileType>>> {
        &self.allowed_neighbors
    }
    fn weights<'a>(&'a self) -> &'a HashMap<TileType, f32> {
        &self.weights
    }
}

impl Default for BuildingRules {
    fn default() -> Self {
        let mut allowed_neighbors = HashMap::new();
        let mut rules_map: HashMap<Direction, Vec<TileType>> = HashMap::new();

        let mut weights = HashMap::new();
        BuildingRules {
            allowed_neighbors,
            all_tiles: vec![TileType::Ground, TileType::Tree, TileType::Chest],
            weights,
        }
    }
}
