use super::*;

impl Cell {
    pub fn new(coordinate: Coordinate) -> Self {
        let num = Number::default();

        Self {
            coordinate,
            num,
            is_template: false,
        }
    }
}

impl Cell {
    pub fn is_filled(&self) -> bool {
        self.num.is_filled()
    }

    pub fn reset(&mut self) {
        self.set_num(Number::N0);
    }

    pub fn set_num_template(&mut self, value: &str) {
        let num = Number::parse(value);
        self.set_num(num);

        if num.is_filled() {
            self.is_template = true;
        }
    }

    pub fn set_num(&mut self, num: Number) {
        if self.is_template {
            return;
        }

        self.num = num;
    }
}
