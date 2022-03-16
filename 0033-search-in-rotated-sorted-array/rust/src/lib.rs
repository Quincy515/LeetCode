struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if target == nums[mid] { return mid as i32;}
            if nums[left] <= nums[mid] && nums[mid] <= nums[right-1] {
                if target < nums[mid] {
                    right = mid
                } else {
                    left = mid + 1
                }
            } else if nums[left] >= nums[mid] {
                if target < nums[mid] || target >= nums[left] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else if nums[mid] >= nums[right -1] {
                if target < nums[mid] && target >= nums[left] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
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
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }
}
