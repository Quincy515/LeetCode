struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut sum = 0;
        let mut min_len = (nums.len() + 1) as i32;

        while right < nums.len() {
            sum += nums[right];
            while sum >= target {
                min_len = min_len.min((right - left + 1) as i32);
                sum -= nums[left];
                left += 1;
            }
            right += 1;
        }
        if min_len <= nums.len() as i32 {
            min_len
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(Solution::min_sub_array_len(target, nums), 2);
    }

    #[test]
    fn test_2() {
        let target = 4;
        let nums = vec![1, 4, 4];
        assert_eq!(Solution::min_sub_array_len(target, nums), 1);
    }

    #[test]
    fn test_3() {
        let target = 11;
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(Solution::min_sub_array_len(target, nums), 0);
    }
}
