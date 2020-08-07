use super::*;

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        let num = Number::default();

        let potentials: Vec<Number> = Vec::new();

        Self { x, y, potentials, num }
    }

    pub fn update(&mut self, value: &str) {
        let num = Number::parse(value);
        self.num = num;
    }
}
