use crate::enums::direction::Direction;
use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub velocity: Point,
    pub current_frame: i32,
    pub facing: Direction,
}
