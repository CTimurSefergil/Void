use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::Direction;

pub fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Front => Direction::Back,
        Direction::Back => Direction::Front,
        Direction::Right => Direction::Left,
        Direction::Left => Direction::Right,
        Direction::FrontRight => Direction::BackLeft,
        Direction::FrontLeft => Direction::BackRight,
        Direction::BackRight => Direction::FrontLeft,
        Direction::BackLeft => Direction::FrontRight,
    }
}
