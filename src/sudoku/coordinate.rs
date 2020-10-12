use std::cmp::Ordering;
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

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.x, self.y).cmp(&(other.x, other.y)))
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
