use bevy::platform::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Ground,
    Tree,
    Chest,
<<<<<<< Updated upstream
    Empty
=======
>>>>>>> Stashed changes
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Direction {
    ZeroZeroOne,
    ZeroZeroMinusOne,
    OneZeroZero,
    MinusOneZeroZero,
}

<<<<<<< Updated upstream
const _DIRECTIONS: [Direction; 4] = [
    Direction::ZeroZeroOne,
    Direction::ZeroZeroMinusOne,
    Direction::OneZeroZero,
    Direction::MinusOneZeroZero,
];

=======
>>>>>>> Stashed changes
pub const DIRECTION_VECTORS: [(Direction, (i32, i32)); 4] = [
    (Direction::ZeroZeroOne, (0, 1)),
    (Direction::ZeroZeroMinusOne, (0, -1)),
    (Direction::OneZeroZero, (1, 0)),
    (Direction::MinusOneZeroZero, (-1, 0)),
];

pub trait Rules {
    fn allowed_neighbors<'a>(&'a self) -> &'a HashMap<TileType, HashMap<Direction, Vec<TileType>>>;
    fn weights<'a>(&'a self) -> &'a HashMap<TileType, f32>;
}
