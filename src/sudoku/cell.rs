use super::*;

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        let num = Number::default();

        let options: Vec<_> = Number::all();

        Self { x, y, num, options, template: false }
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
            self.template = true;
        }
    }

    pub fn set_num(&mut self, num: &Number) {
        if self.template {
            return;
        }

        self.num = num.clone();

        if num != &Number::N0 {
            self.options.clear();
            return;
        }
    }

    pub fn update_options(&mut self, other_nums: Vec<Number>) {
        //let update_options = get_update_options(self, board_info);

        let options: Vec<Number> = Number::all()
            .drain(..)
            .filter(|x| !other_nums.contains(x))
            .collect();

        if options.len() < 1 {
            return;
        }

        self.options = options;

        if self.options.len() == 1 {
            let option = self.options.first().unwrap().to_owned();

            self.set_num(&option);
        }
    }
}
