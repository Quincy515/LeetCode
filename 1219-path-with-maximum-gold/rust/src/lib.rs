impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 {
                    ret = ret.max(Self::backtrack(&mut grid, i, j));
                }
            }
        }
        ret
    }

    fn backtrack(grid: &mut Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        if grid[row][col] == 0 {
            return 0;
        }

        let original = grid[row][col];
        grid[row][col] = 0;

        let left = {
            if col == 0 { 0 } else {
                Self::backtrack(grid, row, col - 1)
            }
        };
        let right = {
            if col == grid[0].len() - 1 { 0 } else {
                Self::backtrack(grid, row, col + 1)
            }
        };
        let up = {
            if row == 0 { 0 } else {
                Self::backtrack(grid, row - 1, col)
            }
        };
        let down = {
            if row == grid.len() - 1 { 0 } else {
                Self::backtrack(grid, row + 1, col)
            }
        };
        grid[row][col] = original;

        let horizontal_max = left.max(right);
        let vertical_max = up.max(down);
        horizontal_max.max(vertical_max) + grid[row][col]
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]]),
            24
        );
        assert_eq!(
            Solution::get_maximum_gold(vec![
                vec![1, 0, 7],
                vec![2, 0, 6],
                vec![3, 4, 5],
                vec![0, 3, 0],
                vec![9, 0, 20]
            ]),
            28
        );
    }
}
