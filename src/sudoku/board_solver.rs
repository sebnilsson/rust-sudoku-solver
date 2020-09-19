use super::Board;

pub fn solve(board: &Board) {
    // TODO: Implement
    let columns = board.columns();
    let rows = board.rows();
    let regions = board.regions();

    println!(
        "Columns: {}, Rows: {}, Regions: {}",
        columns.len(),
        rows.len(),
        regions.len()
    )
}
