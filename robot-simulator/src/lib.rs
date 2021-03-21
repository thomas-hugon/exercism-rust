// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use crate::Direction::{East, North, South, West};

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            d: match self.d {
                Direction::North => East,
                Direction::East => South,
                Direction::South => West,
                Direction::West => North,
            },
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            d: match self.d {
                Direction::North => West,
                Direction::East => North,
                Direction::South => East,
                Direction::West => South,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |r, c| match c {
            'A' => r.advance(),
            'R' => r.turn_right(),
            'L' => r.turn_left(),
            _ => r,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
