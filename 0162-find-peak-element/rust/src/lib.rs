impl Solution {
    // Time: O(n), Space: O(1)
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        if nums.len() == 1 {
            return 0;
        }
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                return i as i32;
            }
        }
        (nums.len() - 1) as i32
    }

    // Time: O(log(n)), Space: O(1)
    pub fn find_peak_element2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut low, mut high) = (0, nums.len() - 1);
        while low < high {
            let mid = low + (high - low) / 2;
            let left = if mid >= 1 {
                nums[mid - 1]
            } else {
                std::i32::MIN
            };
            let right = if mid + 1 < nums.len() {
                nums[mid + 1]
            } else {
                std::i32::MIN
            };
            if nums[mid] > left && nums[mid] > right {
                return mid as i32;
            } else if left > nums[mid] {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        low as i32
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 1);
        assert_eq!(Solution::find_peak_element2(vec![6, 5, 4, 3, 2, 3, 2]), 0);
    }
}
