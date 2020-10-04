use super::*;

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        let num = Number::default();

        let options: Vec<_> = Number::all();

        Self { x, y, num, options, template: false }
    }

    pub fn is_solved(&self) -> bool {
        self.num != Number::N0
    }

    pub fn set_template(&mut self, value: &str) {
        let num = Number::parse(value);
        self.update(num);

        self.template = true;
    }

    pub fn update(&mut self, num: Number) {
        self.num = num;

        if num != Number::N0 {
            self.options.clear();
            return;
        }
    }

    pub fn update_options(&mut self, region_numbers: Vec<Number>) {
        let options = Number::all()
            .drain(..)
            .filter(|x| !region_numbers.contains(x))
            .collect();

        self.options = options;
    }

    pub fn try_remove_option(&mut self, num: &Number) {
        let index = self.options.iter().position(|x| x == num);
        if index.is_some() {
            self.options.remove(index.unwrap());
        }

        if self.options.len() == 1 {
            let option = self.options.first().unwrap();
            self.num = option.to_owned();
        }
    }
}
