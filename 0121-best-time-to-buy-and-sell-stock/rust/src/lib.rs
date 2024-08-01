struct Solution;
impl Solution {
    pub fn max_profit_v1(prices: Vec<i32>) -> i32 {
        let (n, mut max, mut cur_max) = (prices.len(), vec![0; prices.len()], 0);
        for i in (0..=n - 1).rev() {
            max[i] = cur_max;
            if prices[i] > cur_max {
                cur_max = prices[i];
            }
        }
        let mut result = 0;
        for i in 0..n {
            if result < max[i] - prices[i] {
                result = max[i] - prices[i];
            }
        }
        result
    }

    pub fn max_profit_v2(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min = prices[0];

        for p in prices {
            max = max.max(p - min);
            min = min.min(p);
        }

        max
    }

    // 动态规划 https://www.algomooc.com/1563.html
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 先获取数组的长度
        let n = prices.len();
        // 设置一个三位数组 dp
        // dp[i][k][b]
        // i 表示天数，dp[i] 表示第 i 天的最大利润
        // k 表示最多交易次数，每次交易包含买入和卖出，这里只在买入的时候将 k - 1
        // 注意：【 k 表示最多交易次数，而不是实际交易次数，比如最多交易两次可能实际只交易一次】
        // b 表示当前是否持有股票，取值为 0 和 1
        // 其中 0 表示当前持有 0 份股票，即【不持有】股票
        // 而 1 表示当前持有 1 份股票，即【持有】股票

        // 在本题中，k 的值为 1，i 的取值范围为数组 prices 的长度，从 0 开始
        let mut dp = vec![vec![vec![0; 2]; 2]; n];

        // dp[0][0][0] 表示在第 0 天结束时，即收盘后，手上持有 0 份股票，且此时最多进行了 0 次交易的情况下可以获得的最大收益
        // 此时，就是什么都没做，利润为 0
        dp[0][0][0] = 0;

        // dp[0][1][0] 表示在第 0 天结束时，即收盘后，手上持有 0 份股票，且此时最多进行了 1 次交易的情况下可以获得的最大收益
        // 此时，就是什么都没做，利润为 0
        dp[0][1][0] = 0;

        // dp[0][1][1] 表示在第 0 天结束时，即收盘后，手上持有 1 份股票，且此时最多进行了 1 次交易的情况下可以获得的最大收益
        // 手上持有了 1 份股票，那肯定是执行了买入操作，然后又还没有卖出，那么钱都投入了股票中，利润就是负的，即为 -prices[0]
        dp[0][1][1] = -prices[0];

        // 动态规划：自底向上，即从前向后遍历，实现一个萝卜一个坑
        for i in 1..n {
            // 对于每个坑来说，都有两种状态
            // 今天也就是第 i 天

            // 1、今天【不持有】股票
            // 第 i - 1 天【不持有】股票，第 i 天不操作
            // 昨天【不持有】股票，今天不操作
            // vs
            // 第 i - 1 天【持有】股票，第 i 天卖出
            // 昨天【持有】股票，今天卖出
            dp[i][1][0] = i32::max(dp[i - 1][1][0], dp[i - 1][1][1] + prices[i]);

            // 2、今天【持有】股票
            // 第 i - 1 天【持有】股票，第 i 天不操作
            // 昨天【持有】股票，今天不操作
            // vs
            // 第 i - 1 天【不持有】股票，第 i 天买入
            // 昨天【不持有】股票，今天买入
            dp[i][1][1] = i32::max(dp[i - 1][1][1], dp[i - 1][0][0] - prices[i]);
        }
        // for 循环结束后，dp 数组填充完毕
        // dp[length - 1][1][0]
        // 表示第 length - 1 天结束时，即收盘后，手上持有 0 份股票，且此时最多进行了 1 次交易的情况下可以获得的最大收益
        dp[n - 1][1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
