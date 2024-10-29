#![allow(dead_code)]

/*

You are given a 0-indexed m x n matrix grid consisting of positive integers.

You can start at any cell in the first column of the matrix, and traverse the grid in the following way:

    From a cell (row, col), you can move to any of the cells: (row - 1, col + 1), (row, col + 1) and (row + 1, col + 1)
    such that the value of the cell you move to, should be strictly bigger than the value of the current cell.

Return the maximum number of moves that you can perform.

*/

fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![-1; n]; m];
    let mut max_moves = 0;

    fn dfs(grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        if dp[row][col] != -1 {
            return dp[row][col];
        }

        let directions = [(-1, 1), (0, 1), (1, 1)];
        let mut max_move = 0;

        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < grid.len() as i32 && new_col < grid[0].len() as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                if grid[new_row][new_col] > grid[row][col] {
                    max_move = max_move.max(1 + dfs(grid, dp, new_row, new_col));
                }
            }
        }

        dp[row][col] = max_move;
        max_move
    }

    for row in 0..m {
        max_moves = max_moves.max(dfs(&grid, &mut dp, row, 0));
    }

    max_moves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];
        assert_eq!(max_moves(grid), 3);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        assert_eq!(max_moves(grid), 0);
    }

    #[test]
    fn test_single_row_no_moves() {
        let grid = vec![vec![1, 2, 3, 4, 5]];
        assert_eq!(max_moves(grid), 4);
    }

    #[test]
    fn test_single_column_no_moves() {
        let grid = vec![vec![1], vec![2], vec![3], vec![4]];
        assert_eq!(max_moves(grid), 0);
    }

    #[test]
    fn test_large_values_with_moves() {
        let grid = vec![vec![100, 200, 300], vec![50, 250, 400], vec![25, 150, 350]];
        assert_eq!(max_moves(grid), 2);
    }

    #[test]
    fn test_no_possible_moves() {
        let grid = vec![vec![5, 4, 3], vec![4, 3, 2], vec![3, 2, 1]];
        assert_eq!(max_moves(grid), 0);
    }

    #[test]
    fn test_maximum_moves_possible() {
        let grid = vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4]];
        assert_eq!(max_moves(grid), 3);
    }

    #[test]
    fn test_two_rows_multiple_paths() {
        let grid = vec![vec![1, 3, 5, 7], vec![2, 4, 6, 8]];
        assert_eq!(max_moves(grid), 3);
    }

    #[test]
    fn test_large_grid() {
        let grid = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
            vec![4, 5, 6, 7, 8],
            vec![5, 6, 7, 8, 9],
        ];
        assert_eq!(max_moves(grid), 4);
    }
}
