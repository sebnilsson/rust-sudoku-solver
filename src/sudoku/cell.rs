use super::*;

impl Cell {
    pub fn new(coordinate: Coordinate) -> Self {
        let num = Number::default();

        let potentials: Vec<_> = Number::all();

        Self { coordinate, num, potentials, template: false }
    }
}

impl Cell {
    // pub fn is_empty(&self) -> bool {
    //     self.num == Number::N0
    // }

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
            self.potentials.clear();
            return;
        }
    }

    pub fn update_potentials(&mut self, region_nums: Vec<Number>) {
        let potentials: Vec<Number> = Number::all()
            .drain(..)
            .filter(|x| !region_nums.contains(x))
            .collect();

        if potentials.len() < 1 {
            return;
        }

        self.potentials = potentials;

        if self.potentials.len() == 1 {
            let potential = self.potentials.first().unwrap().to_owned();

            self.set_num(&potential);
        }
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}
