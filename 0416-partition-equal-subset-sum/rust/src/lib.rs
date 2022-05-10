struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut sum = 0;
        for i in 0..n {
            sum += nums[i];
        }
        if sum % 2 == 1 {
            return false;
        }
        sum /= 2;
        let mut dp = vec![vec![false; sum as usize + 1]; n];
        dp[0][0] = true;
        if nums[0] <= sum {
            dp[0][nums[0] as usize] = true;
        }
        for i in 1..n {
            for j in 0..=sum {
                if j - nums[i] >= 0 {
                    dp[i as usize][j as usize] = dp[i as usize - 1][j as usize]
                        || dp[i as usize - 1][(j - nums[i]) as usize];
                } else {
                    dp[i as usize][j as usize] = dp[i as usize - 1][j as usize];
                }
            }
        }
        dp[n as usize - 1][sum as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
    }
}
