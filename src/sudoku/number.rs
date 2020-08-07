use super::*;

impl Default for Number {
    fn default() -> Self {
        Number::N0
    }
}

impl Number {
    pub fn parse(value: &str) -> Self {
        match value {
            "1" => Number::N1,
            "2" => Number::N2,
            "3" => Number::N3,
            "4" => Number::N4,
            "5" => Number::N5,
            "6" => Number::N6,
            "7" => Number::N7,
            "8" => Number::N8,
            "9" => Number::N9,
            _ => Number::default(),
        }
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
