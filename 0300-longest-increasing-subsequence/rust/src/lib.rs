struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        dp[0] = 1;
        for i in 1..n {
            dp[i] = 1;
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = i32::max(dp[i], dp[j] + 1);
                }
            }
        }

        let mut result = 0;
        for i in 0..n {
            if dp[i] > result {
                result = dp[i];
            }
        }
        result
    }

    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut list_to_min_v = vec![0; n + 1];
        let mut k = 0i32;

        let mut dp = vec![0; n];
        for i in 0..n {
            let len = Self::bsearch(&mut list_to_min_v, k, nums[i]);
            if len == -1 {
                dp[i] = 1;
            } else {
                dp[i] = len + 1;
            }
            if dp[i] > k {
                k = dp[i];
                list_to_min_v[dp[i] as usize] = nums[i];
            } else if list_to_min_v[dp[i] as usize] > nums[i] {
                list_to_min_v[dp[i] as usize] = nums[i];
            }
        }

        let mut result = 0;
        for i in 0..n {
            if dp[i] > result {
                result = dp[i];
            }
        }
        result
    }

    // 查找最后一个比 target 小的元素位置
    fn bsearch(a: &mut Vec<i32>, k: i32, target: i32) -> i32 {
        let (mut low, mut high) = (1, k);
        while low <= high {
            let mid = low + (high - low) / 2;
            if a[mid as usize] < target {
                if mid == k || a[mid as usize + 1] >= target {
                    return mid;
                } else {
                    low = mid + 1;
                }
            } else {
                high = mid - 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::length_of_lis2(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
        assert_eq!(Solution::length_of_lis2(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis2(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
