use bevy::platform::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter)]
pub enum TileType {
    Ground, 
    Tree,   
    Chest,

    FountainCenter,  
    FountainCorner1, 
    FountainCorner2, 
    FountainCorner3, 
    FountainCorner4, 
    FountainEdge1,   
    FountainEdge2,   
    FountainEdge3,   
    FountainEdge4,   
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    Front, 
    Back,  
    Right, 
    Left,  
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Front,
    Direction::Back,
    Direction::Right,
    Direction::Left,
];

pub const DIRECTION_VECTORS: [(Direction, (i32, i32)); 4] = [
    (Direction::Front, (0, 1)), // +Z
    (Direction::Back, (0, -1)), // -Z
    (Direction::Right, (1, 0)), // +X
    (Direction::Left, (-1, 0)), // -X
];

pub trait Rules {
    fn allowed_neighbors<'a>(&'a self) -> &'a HashMap<TileType, HashMap<Direction, Vec<TileType>>>;
    fn weights<'a>(&'a self) -> &'a HashMap<TileType, f32>;
}
