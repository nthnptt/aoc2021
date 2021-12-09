pub fn extract_low_points(grid: Vec<Vec<u32>>) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    for (r_index, row) in grid.iter().enumerate() {
        for (c_index, cell) in row.iter().enumerate() {
            let mut is_lower = true;
            if c_index > 0 {
                is_lower &= cell < &row[c_index - 1];
            }
            if c_index < row.len() - 1 {
                is_lower &= cell < &row[c_index + 1];
            }
            if r_index > 0 {
                is_lower &= cell < &grid[r_index - 1][c_index];
            }
            if r_index < grid.len() - 1 {
                is_lower &= cell < &grid[r_index + 1][c_index];
            }
            if is_lower {
                result.push(*cell);
            }
        }
    }
    result
}
