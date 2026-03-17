use rand::Rng;

pub const NUM_COLS: usize = 53;
pub const NUM_ROWS: usize = 7;

#[derive(Debug, Clone)]
pub struct Cell {
    #[allow(dead_code)]
    pub col: usize,
    #[allow(dead_code)]
    pub row: usize,
    pub level: u8,
}

pub type Grid = Vec<Vec<Option<Cell>>>;

pub fn generate_grid() -> Grid {
    let mut rng = rand::thread_rng();
    let mut grid: Grid = Vec::with_capacity(NUM_COLS);
    for col in 0..NUM_COLS {
        let mut column = Vec::with_capacity(NUM_ROWS);
        for row in 0..NUM_ROWS {
            let r: f64 = rng.gen();
            let level = if r < 0.50 {
                0
            } else if r < 0.70 {
                1
            } else if r < 0.85 {
                2
            } else if r < 0.95 {
                3
            } else {
                4
            };
            column.push(Some(Cell { col, row, level }));
        }
        grid.push(column);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_dimensions() {
        let grid = generate_grid();
        assert_eq!(grid.len(), NUM_COLS);
        for col in &grid {
            assert_eq!(col.len(), NUM_ROWS);
        }
    }

    #[test]
    fn test_all_cells_are_some_with_valid_levels() {
        let grid = generate_grid();
        for col in 0..NUM_COLS {
            for row in 0..NUM_ROWS {
                let cell = grid[col][row].as_ref().expect("Cell should be Some");
                assert_eq!(cell.col, col);
                assert_eq!(cell.row, row);
                assert!(cell.level <= 4, "Level should be 0-4, got {}", cell.level);
            }
        }
    }

    #[test]
    fn test_level_distribution() {
        let mut counts = [0u32; 5];
        // Generate many grids to get statistical significance
        for _ in 0..100 {
            let grid = generate_grid();
            for col in &grid {
                for cell_opt in col {
                    if let Some(cell) = cell_opt {
                        counts[cell.level as usize] += 1;
                    }
                }
            }
        }
        let total: u32 = counts.iter().sum();
        let pcts: Vec<f64> = counts.iter().map(|&c| c as f64 / total as f64).collect();
        // Level 0 should be most common (around 50%)
        assert!(pcts[0] > 0.35, "Level 0 should be most common, got {:.2}%", pcts[0] * 100.0);
        // Level 4 should be least common (around 5%)
        assert!(pcts[4] < 0.15, "Level 4 should be rare, got {:.2}%", pcts[4] * 100.0);
        // Level 0 > Level 4
        assert!(pcts[0] > pcts[4]);
    }
}
