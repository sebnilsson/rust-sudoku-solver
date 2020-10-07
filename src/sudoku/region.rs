use super::*;

impl<'a> Region<'a> {
    pub fn new() -> Self {
        let cells: Vec<_> = Vec::new();
        Self { cells }
    }
}
