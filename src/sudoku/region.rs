use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }
}

// impl<'a> Region<'a> {
//     pub fn nums(&self) -> Vec<Number> {
//         self.cells
//             .iter()
//             .filter(|x| !x.borrow().is_empty())
//             .map(|x| x.borrow().num)
//             .collect()
//     }
// }
