struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if target > nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if nums[left] != target {
            return vec![-1, -1];
        }
        let start = left as i32;
        while left < nums.len() && nums[left] == target {
            left += 1;
        }
        vec![start, (left - 1) as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::search_range(vec![2, 2], 3), vec![-1, -1]);
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
