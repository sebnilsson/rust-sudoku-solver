use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<&_> = Vec::new();
        Self { cells }
    }

    pub fn from(cells: Vec<&'a BoardCell>) -> Self {
        let mut region = Region::new();
        for x in cells {
            region.cells.push(x);
        }
        region
    }

    pub fn recalc_options(&mut self) {
        let unsolved_cells: Vec<&&BoardCell> = self
            .cells
            .iter()
            .filter(|x| x.borrow().num != Number::N0)
            .collect();
        for cell in unsolved_cells.iter() {
            cell.borrow_mut().options = Number::all();
        }

        for num in Number::all() {
            let num_exists = self.cells.iter().any(|x| x.borrow().num == num);
            if !num_exists {
                continue;
            }

            for cell in unsolved_cells.iter() {
                let index =
                    cell.borrow().options.iter().position(|x| x == &num);

                if index.is_some() {
                    cell.borrow_mut().options.remove(index.unwrap());
                }
            }
        }
    }
}
