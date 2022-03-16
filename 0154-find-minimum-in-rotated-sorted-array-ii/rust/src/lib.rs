impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else if nums[mid] < nums[right] {
                right = mid;
            } else {
                right -= 1;
            }
        }
        nums[left]
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_min(vec![3, 1, 1]), 1);
        assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }
}
