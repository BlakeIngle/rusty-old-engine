use sdl2::rect::Point;

use crate::enums::direction::Direction;

use self::Direction::*;

pub fn direction_sprite_sheet_row(direction: Direction) -> i32 {
    match direction {
        Down => 0,
        Left => 1,
        Right => 2,
        Up => 3,
    }
}

pub fn direction_from_velocity(velocity: &Point) -> Option<Direction> {
    if velocity.y > 0 {
        Some(Down)
    } else if velocity.y < 0 {
        Some(Up)
    } else if velocity.x > 0 {
        Some(Right)
    } else if velocity.x < 0 {
        Some(Left)
    } else {
        None
    }
}
