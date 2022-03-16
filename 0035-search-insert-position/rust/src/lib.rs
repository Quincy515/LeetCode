struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right -left) /2;
            if target == nums[mid] { return mid as i32; }
            else if target < nums[mid] { 
                right = mid;
            } else if target > nums[mid] {
                left = mid + 1;
            }
        }
        left as i32
    }

    pub fn search_insert2(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert2(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert2(vec![1], 0), 0);
    }
}
