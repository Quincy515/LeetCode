struct Solution;

impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid.first().map_or(0, Vec::len));
        let mut dp = vec![vec![0; m]; n];
        let mut sum = 0;
        for j in 0..m {
            sum += grid[0][j];
            dp[0][j] = sum;
        }
        sum = 0;
        for i in 0..n {
            sum += grid[i][0];
            dp[i][0] = sum;
        }

        for i in 1..n {
            for j in 1..m {
                dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[n - 1][m - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_value(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            12
        );
    }
}
