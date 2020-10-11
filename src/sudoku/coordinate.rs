use std::fmt;

use super::*;

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Coordinate { x, y }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
