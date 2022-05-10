struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid.first().map_or(0, Vec::len));
        let mut dp = vec![vec![0; n]; m];
        let mut len = 0;
        for i in 0..m {
            len += grid[i][0];
            dp[i][0] = len;
        }
        len = 0;
        for j in 0..n {
            len += grid[0][j];
            dp[0][j] = len;
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }
}
