use crate::game::core_mechanics::oz_devinimli_yaratim::odyrules::commons::Direction;

pub fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::ZeroZeroOne => Direction::ZeroZeroMinusOne,
        Direction::ZeroZeroMinusOne => Direction::ZeroZeroOne,
        Direction::OneZeroZero => Direction::MinusOneZeroZero,
        Direction::MinusOneZeroZero => Direction::OneZeroZero,
    }
}
