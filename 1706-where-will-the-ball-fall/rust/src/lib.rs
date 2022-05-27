struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let col = grid.first().map_or(0, Vec::len);
        let mut result = vec![0; col];
        for i in 0..col {
            result[i] = dfs(&grid, 0, i);
        }
        result
    }
}

fn dfs(grid: &[Vec<i32>], i: usize, j: usize) -> i32 {
    if i == grid.len() {
        return j as i32;
    }
    if grid[i][j] == 1 {
        if j == grid[0].len() - 1 || grid[i][j + 1] == -1 {
            return -1;
        }
        dfs(grid, i + 1, j + 1)
    } else {
        if j == 0 || grid[i][j - 1] == 1 {
            return -1;
        }
        dfs(grid, i + 1, j - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_ball(vec![vec![-1]]), vec![-1]);
        assert_eq!(
            Solution::find_ball(vec![
                vec![1, 1, 1, -1, -1],
                vec![1, 1, 1, -1, -1],
                vec![-1, -1, -1, 1, 1],
                vec![1, 1, 1, 1, -1],
                vec![-1, -1, -1, -1, -1]
            ]),
            vec![1, -1, -1, -1, -1]
        );
        assert_eq!(
            Solution::find_ball(vec![
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1],
                vec![1, 1, 1, 1, 1, 1],
                vec![-1, -1, -1, -1, -1, -1]
            ]),
            vec![0, 1, 2, 3, 4, -1]
        );
    }
}
