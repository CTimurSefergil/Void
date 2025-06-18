use bevy::{
    ecs::{component::Component, resource::Resource},
    platform::collections::HashMap,
};

use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::{
    Direction, Rules, TileType,
};

#[derive(Component)]
pub struct Ground;
#[derive(Component)]
pub struct Tree;
#[derive(Component)]
pub struct Chest;

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

        OpenSpaceRules {
            allowed_neighbors,
            all_tiles: vec![TileType::Ground, TileType::Tree, TileType::Chest],
            weights,
        }
    }
}
