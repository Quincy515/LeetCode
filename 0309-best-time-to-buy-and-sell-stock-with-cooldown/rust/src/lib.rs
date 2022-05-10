struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let mut dp = vec![vec![0; 4]; n];
        // dp[i][0] 表示第 i 天持有股票时的利润
        // dp[i][1] 表示第 i 天不持有股票时的利润（当天刚卖掉）
        // dp[i][2] 表示第 i 天不持有股票时的利润（冷冻期），昨天刚卖了股票
        // dp[i][3] 表示第 i 天不持有股票时的利润（非冷冻期），昨天也没有持有
        dp[0][0] = -prices[0];
        dp[0][1] = 0;
        dp[0][2] = 0;
        dp[0][3] = 0;
        for i in 1..n {
            dp[i][0] = dp[i - 1][0]
                .max(dp[i - 1][2] - prices[i])
                .max(dp[i - 1][3] - prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = dp[i - 1][1];
            dp[i][3] = i32::max(dp[i - 1][2], dp[i - 1][3]);
        }
        dp[n - 1][0]
            .max(dp[n - 1][1])
            .max(dp[n - 1][2])
            .max(dp[n - 1][3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
