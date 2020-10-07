use super::*;

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        let num = Number::default();

        let options: Vec<_> = Number::all();

        Self { x, y, num, options, template: false, guess: false }
    }

    pub fn index(&self) -> usize {
        Board::index(self.x, self.y)
    }

    pub fn is_solved(&self) -> bool {
        self.num != Number::N0
    }

    pub fn reset_guess(&mut self) {
        self.set_num(&Number::N0);

        self.guess = false;
    }

    pub fn set_num_guess(&mut self, num: &Number) {
        self.set_num(num);

        self.guess = true;
    }

    pub fn set_num_solution(&mut self, num: &Number) {
        self.set_num(num);

        self.guess = false;
    }

    pub fn set_num_template(&mut self, value: &str) {
        let num = Number::parse(value);
        self.set_num(&num);

        if num != Number::N0 {
            self.template = true;
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
            let option = self.options.first().unwrap().to_owned();
            
            self.set_num_solution(&option);
        }
    }

    fn set_num(&mut self, num: &Number) {
        if self.template {
            return;
        }

        self.num = num.clone();

        if num != &Number::N0 {
            self.options.clear();
            return;
        }
    }
}
