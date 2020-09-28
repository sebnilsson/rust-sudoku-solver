use std::cmp::Ordering;

use super::*;

impl Number {
    pub fn parse(value: &str) -> Self {
        let parsed = value.parse::<u8>().unwrap_or_default();
        Number::map(parsed)
    }

    pub fn map(value: u8) -> Self {
        match value {
            1 => Number::N1,
            2 => Number::N2,
            3 => Number::N3,
            4 => Number::N4,
            5 => Number::N5,
            6 => Number::N6,
            7 => Number::N7,
            8 => Number::N8,
            9 => Number::N9,
            _ => Number::default(),
        }
    }

    pub fn from_ref(value: &Self) -> Self {
        match value {
            Number::N0 => Number::N0,
            Number::N1 => Number::N1,
            Number::N2 => Number::N2,
            Number::N3 => Number::N3,
            Number::N4 => Number::N4,
            Number::N5 => Number::N5,
            Number::N6 => Number::N6,
            Number::N7 => Number::N7,
            Number::N8 => Number::N8,
            Number::N9 => Number::N9,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Number::N1,
            Number::N2,
            Number::N3,
            Number::N4,
            Number::N5,
            Number::N6,
            Number::N7,
            Number::N8,
            Number::N9,
        ]
    }
}

impl Number {
    pub fn to_usize(&self) -> usize {
        match self {
            Number::N0 => 0,
            Number::N1 => 1,
            Number::N2 => 2,
            Number::N3 => 3,
            Number::N4 => 4,
            Number::N5 => 5,
            Number::N6 => 6,
            Number::N7 => 7,
            Number::N8 => 8,
            Number::N9 => 9,
        }
    }

    pub fn to_str(&self) -> String {
        let usize = self.to_usize();
        if usize <= 0 {
            return String::from("_");
        }

        format!("{}", usize)
    }
}

impl Default for Number {
    fn default() -> Self {
        Number::N0
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_usize())
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_usize().cmp(&other.to_usize()))
    }
}
