struct Solution;

impl Solution {
    // 动态规划
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = i32::MIN;
        let mut dp = vec![0; n];
        // base case
        // 第一个元素前面没有子数组
        dp[0] = nums[0];
        // 状态转移方程
        for i in 1..n {
            dp[i] = nums[i].max(nums[i] + dp[i - 1]);
        }
        // 得到 nums 的最大子数组
        for i in 0..n {
            result = result.max(dp[i]);
        }
        result
    }

    // 滑动窗口
    pub fn max_sub_array_windows(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_sum = i32::MIN;
        let mut sum = 0;
        for i in 0..n {
            if sum < 0 {
                sum = 0;
            }
            sum += nums[i];
            if sum > max_sum {
                max_sum = sum;
            }
        }
        max_sum
    }
    // 前后缀
    pub fn max_sub_array2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut sum, mut max) = (vec![0; nums.len()], vec![0; nums.len()]);

        let mut cursum = 0;
        for i in 0..nums.len() {
            cursum += nums[i];
            sum[i] = cursum;
        }
        let mut curmax = i32::MIN;
        for i in (0..=sum.len() - 1).rev() {
            if curmax < sum[i] {
                curmax = sum[i];
            }
            max[i] = curmax;
        }
        let mut result = i32::MIN;
        for i in 0..nums.len() {
            if i == 0 && result < max[0] {
                result = max[0];
            }
            if i != 0 && result < max[i] - sum[i - 1] {
                result = max[i] - sum[i - 1];
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array2(vec![5, 4, -1, 7, 8]), 23);
    }
}
