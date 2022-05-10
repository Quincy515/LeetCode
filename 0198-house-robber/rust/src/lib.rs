struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        // dp[i][0] 表示第 i 个物品没有选时的最大金额
        // dp[i][1] 表示第 i 个物品选择时的最大金额
        let mut dp = vec![vec![0; 2]; n];
        dp[0][0] = 0;
        dp[0][1] = nums[0];
        for i in 1..n {
            dp[i][0] = i32::max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        i32::max(dp[n - 1][0], dp[n - 1][1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
