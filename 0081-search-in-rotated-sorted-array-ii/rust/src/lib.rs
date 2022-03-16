impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if target == nums[mid] {
                return true;
            }
            if nums[left] == nums[mid] && nums[mid] == nums[right - 1] {
                left += 1;
                right -= 1;
            } else if nums[left] <= nums[mid] {
                if target < nums[mid] && target >= nums[left] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if target < nums[mid] || target > *nums.last().unwrap() {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }

        false
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    }
}
