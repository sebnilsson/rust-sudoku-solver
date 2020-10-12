use super::*;

impl Cell {
    pub fn new(coordinate: Coordinate) -> Self {
        let num = Number::default();

        Self { coordinate, num, is_template: false }
    }
}

impl Cell {
    pub fn is_solved(&self) -> bool {
        self.num != Number::N0
    }

    pub fn reset(&mut self) {
        self.set_num(&Number::N0);
    }

    pub fn set_num_template(&mut self, value: &str) {
        let num = Number::parse(value);
        self.set_num(&num);

        if num != Number::N0 {
            self.is_template = true;
        }
    }

    pub fn set_num(&mut self, num: &Number) {
        if self.is_template {
            return;
        }

        self.num = num.clone();
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}
