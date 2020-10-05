use super::*;

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        let num = Number::default();

        let options: Vec<_> = Number::all();

        Self { x, y, num, options, template: false }
    }

    pub fn index(&self) -> usize {
        Board::index(self.x, self.y)
    }

    pub fn is_solved(&self) -> bool {
        self.num != Number::N0
    }

    pub fn set_template(&mut self, value: &str) {
        let num = Number::parse(value);
        self.update(num);

        if num != Number::N0 {
            self.template = true;
        }
    }

    pub fn update(&mut self, num: Number) {
        if self.template {
            return;
        }

        self.num = num;

        if num != Number::N0 {
            self.options.clear();
            return;
        }
    }

    pub fn update_options(&mut self, region_numbers: &Vec<Number>) {
        let options: Vec<Number> = Number::all()
            .drain(..)
            .filter(|x| !region_numbers.contains(x))
            .collect();

        if options.len() < 1 {
            return;
        }

        self.options = options;

        if self.options.len() == 1 {
            let option = self.options.first().unwrap();
            self.num = option.to_owned();
        }
    }
}
