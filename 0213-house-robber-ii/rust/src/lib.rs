struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return i32::max(nums[0], nums[1]);
        }

        // 第 0 个不偷窃，偷窃 1~n-1 之间的房子
        let max1 = Self::rob_dp(&nums, 1, n - 1);
        // 第 0 个偷窃，偷窃 2~n-1 之间的房子
        let max2 = nums[0] + Self::rob_dp(&nums, 2, n - 2);
        i32::max(max1, max2)
    }

    fn rob_dp(nums: &[i32], p: usize, r: usize) -> i32 {
        let n = nums.len();
        // dp[i][0] 表示第 i 个物品美誉哦选时的最大金额
        // dp[i][1] 表示第 i 个物品选择时的最大金额
        let mut dp = vec![vec![0; 2]; n];
        dp[p][0] = 0;
        dp[p][1] = nums[p];
        for i in p + 1..=r {
            dp[i][0] = i32::max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        i32::max(dp[r][0], dp[r][1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 2]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }
}
