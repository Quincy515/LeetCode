struct Solution;
impl Solution {
    // 大于等于 target 的最小下标
    fn bsearch(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = nums.len();
        if nums.is_empty() {
            return result as i32;
        }

        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] >= target {
                result = mid;
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        result as i32
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let index = Self::bsearch(nums.clone(), target);
        println!("index: {index}");
        if index == nums.len() as i32 || nums[index as usize] != target {
            return 0;
        }
        let index1 = Self::bsearch(nums, target + 1);
        println!("index1: {index1}");
        index1 - index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 8), 2);
        // assert_eq!(Solution::search(vec![5, 7, 7, 8, 8, 10], 6), 0);
        // assert_eq!(Solution::search(vec![2, 2], 1), 0);
        assert_eq!(Solution::search(vec![2, 2], 2), 2);
        assert_eq!(Solution::search(vec![], 0), 0);
        assert_eq!(Solution::search(vec![1], 1), 1);
    }
}
