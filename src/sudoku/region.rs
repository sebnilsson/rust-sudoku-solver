use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }

    // pub fn copy(region: &Self) -> Self {
    //     let cells = region.cells;

    //     let region = Region::new();
    //     region.cells = cells;

    //     region
    // }

    // pub fn from(cells: Vec<&'a Cell>) -> Self {
    //     let mut region = Region::new();
    //     for x in cells {
    //         region.cells.push(x);
    //     }
    //     region
    // }

    // pub fn add_cell(&mut self, cell: &'a mut Cell){
    //     self.cells.push(cell);
    // }

    // pub fn set_cells(&mut self, cells: Vec<Cell>) {
    //     self.cells.clear();

    //     let mut new_cells: Vec<Cell> = Vec::new();

    //     for cell in cells {
    //         new_cells.push(cell);
    //     }

    //     /*let cells: Vec<BoardCell> =
    //         cells
    //         .iter()
    //         .map(|x|x.into_inner())
    //         .map(|x|BoardCell::new(x))
    //         .collect();*/
    //     // TODO: Fix
    //     //self.cells = new_cells;
    // }

    // pub fn recalc_options(&mut self) {
    //     let mut unsolved_cells: Vec<&Cell> =
    //         self.cells.drain(..).filter(|x| x.num != Number::N0).collect();

    //     for x in self.cells.drain(..) {
    //         x.reset_options();
    //     }

    //     for cell in unsolved_cells.iter_mut() {
    //         cell.reset_options();
    //     }

    //     for num in Number::all() {
    //         let num_exists = self.cells.iter().any(|x| x.num == num);
    //         if !num_exists {
    //             continue;
    //         }

    //         for cell in unsolved_cells.iter() {
    //             let index = cell.options.iter().position(|x| x == &num);

    //             if index.is_some() {
    //                 // TODO: Fix
    //                 //cell.options.remove(index.unwrap());
    //             }
    //         }
    //     }
    // }
}
