use super::*;

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        let num = Number::default();

        let options: Vec<_> = Number::all();

        Self { x, y, options, num }
    }

    pub fn is_solved(&self) -> bool {
        self.num != Number::N0
    }

    pub fn update(&mut self, value: &str) {
        let num = Number::parse(value);
        self.num = num;

        let index = self.options.iter().position(|x| *x == num);
        if index.is_some() {
            self.options.remove(index.unwrap());
        }
    }
}
