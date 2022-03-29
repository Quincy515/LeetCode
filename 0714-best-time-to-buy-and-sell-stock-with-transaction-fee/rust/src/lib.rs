struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![0; 2]; n];
        dp[0][0] = -prices[0]; // 第 i 天后持有股票，手里利润的最大值
        dp[0][1] = 0; // 第 i 天后不持有股票，手里利润的最大值
        for i in 1..n {
            dp[i][0] = i32::max(dp[i - 1][0], dp[i - 1][1] - prices[i]);
            dp[i][1] = i32::max(dp[i - 1][0] + prices[i] - fee, dp[i - 1][1]);
        }
        i32::max(dp[n - 1][0], dp[n - 1][1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    }
}
