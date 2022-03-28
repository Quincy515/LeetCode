struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        if target > 1000 || target < -1000 {
            return 0;
        }
        let n = nums.len();
        let offset = 1000;
        let w = 2000;
        let mut dp = vec![vec![0; w + 1]; n];
        dp[0][(offset - nums[0]) as usize] += 1; // 因为 nums[0] 有可能为 0
        dp[0][(offset + nums[0]) as usize] += 1;
        for i in 1..n {
            for j in 0..=w {
                if j as i32 - nums[i] >= 0 && j as i32 - nums[i] <= w as i32 {
                    dp[i][j] = dp[i - 1][j - nums[i] as usize];
                }
                if j as i32 + nums[i] >= 0 && j as i32 + nums[i] <= w as i32 {
                    dp[i][j] += dp[i - 1][j + nums[i] as usize];
                }
            }
        }

        dp[n - 1][target as usize + 1000]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }
}
