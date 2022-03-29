struct Solution;
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![vec![0; n]; n];
        dp[0][0] = triangle[0][0];
        for i in 1..n {
            dp[i][0] = dp[i - 1][0] + triangle[i][0];
            for j in 1..i {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i - 1][j - 1]) + triangle[i][j];
            }
            dp[i][i] = dp[i - 1][i - 1] + triangle[i][i];
        }

        let mut res = i32::MAX;
        for j in 0..n {
            if dp[n - 1][j] < res {
                res = dp[n - 1][j];
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
