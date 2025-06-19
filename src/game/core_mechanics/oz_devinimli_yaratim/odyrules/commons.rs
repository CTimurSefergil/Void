use bevy::platform::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
    Empty,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    Front,
    Back,
    Right,
    Left,
    FrontRight,
    FrontLeft,
    BackRight,
    BackLeft,
}

const _DIRECTIONS: [Direction; 8] = [
    Direction::Front,
    Direction::Back,
    Direction::Right,
    Direction::Left,
    Direction::FrontRight,
    Direction::FrontLeft,
    Direction::BackRight,
    Direction::BackLeft,
];

pub const DIRECTION_VECTORS: [(Direction, (i32, i32)); 8] = [
    (Direction::Front, (0, 1)),
    (Direction::Back, (0, -1)),
    (Direction::Right, (1, 0)),
    (Direction::Left, (-1, 0)),
    (Direction::FrontRight, (1, 1)),
    (Direction::FrontLeft, (-1, 1)),
    (Direction::BackRight, (1, -1)),
    (Direction::BackLeft, (-1, -1)),
];

pub trait Rules {
    fn allowed_neighbors<'a>(&'a self) -> &'a HashMap<TileType, HashMap<Direction, Vec<TileType>>>;
    fn weights<'a>(&'a self) -> &'a HashMap<TileType, f32>;
}
