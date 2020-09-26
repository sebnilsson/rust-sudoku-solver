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

    pub fn try_remove_option(&mut self, num: &Number) {
        let index = self.options.iter().position(|x| x == num);
        if index.is_some() {
            self.options.remove(index.unwrap());
        }

        if self.options.len() == 1 {
            let option = self.options.first().unwrap();
            self.num = Number::from_ref(option);
        }
    }

    pub fn update(&mut self, value: &str) {
        let num = Number::parse(value);
        self.num = num;

        if num != Number::N0 {
            self.options.clear();
            return;
        }
    }
}
