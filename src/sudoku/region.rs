use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }

    // pub fn contains(&self, number: &Number) -> bool {
    //     self.cells.iter().any(|x| &x.borrow().num == number)
    // }
}
